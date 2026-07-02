use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use reqwest::multipart;
use crate::tinytex::find_pdflatex;

const ONLINE_HOSTS: &[&str] = &[
    "https://latexonline.cc",
    "https://texlive2020.latexonline.cc",
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompileResult {
    pub success: bool,
    pub pdf_path: Option<String>,
    pub log: String,
    pub error: Option<String>,
}

pub fn compile_to_pdf(latex_source: &str, output_folder: &str) -> CompileResult {
    let pdflatex = match find_pdflatex() {
        Some(path) => path,
        None => {
            return CompileResult {
                success: false,
                pdf_path: None,
                log: String::new(),
                error: Some("pdfLaTeX not found. Please install TinyTeX in Settings first.".to_string()),
            };
        }
    };

    let temp_dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(e) => {
            return CompileResult {
                success: false,
                pdf_path: None,
                log: String::new(),
                error: Some(format!("Failed to create temp dir: {}", e)),
            };
        }
    };

    let tex_path = temp_dir.path().join("resume.tex");
    let pdf_path = temp_dir.path().join("resume.pdf");
    let log_path = temp_dir.path().join("resume.log");

    if let Err(e) = fs::write(&tex_path, latex_source) {
        return CompileResult {
            success: false,
            pdf_path: None,
            log: String::new(),
            error: Some(format!("Failed to write .tex file: {}", e)),
        };
    }

    let mut full_log = String::new();

    for pass in 1..=2 {
        let output = Command::new(&pdflatex)
            .args([
                "-interaction=nonstopmode",
                tex_path.to_str().unwrap(),
            ])
            .current_dir(temp_dir.path())
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                full_log.push_str(&format!("--- Pass {} ---\n", pass));
                full_log.push_str(&stdout);
                if !stderr.is_empty() {
                    full_log.push_str(&stderr);
                }
            }
            Err(e) => {
                return CompileResult {
                    success: false,
                    pdf_path: None,
                    log: full_log,
                    error: Some(format!("pdflatex execution error: {}", e)),
                };
            }
        }
    }

    if log_path.exists() {
        if let Ok(log_content) = fs::read_to_string(&log_path) {
            full_log.push_str("\n--- LaTeX Log ---\n");
            full_log.push_str(&log_content);
        }
    }

    if pdf_path.exists() {
        let output_dir = PathBuf::from(output_folder);
        if let Err(e) = fs::create_dir_all(&output_dir) {
            return CompileResult {
                success: false,
                pdf_path: None,
                log: full_log,
                error: Some(format!("Failed to create output folder: {}", e)),
            };
        }

        let dest_pdf = output_dir.join("Resume.pdf");
        if dest_pdf.exists() {
            fs::remove_file(&dest_pdf).ok();
        }

        if let Err(e) = fs::copy(&pdf_path, &dest_pdf) {
            return CompileResult {
                success: false,
                pdf_path: None,
                log: full_log,
                error: Some(format!("Failed to copy PDF: {}", e)),
            };
        }

        return CompileResult {
            success: true,
            pdf_path: Some(dest_pdf.to_string_lossy().to_string()),
            log: full_log,
            error: None,
        };
    }

    CompileResult {
        success: false,
        pdf_path: None,
        log: full_log.clone(),
        error: Some("PDF was not generated. Check the log for LaTeX errors.".to_string()),
    }
}

pub fn try_compile_offline(latex_source: &str, output_folder: &str) -> CompileResult {
    let result = compile_to_pdf(latex_source, output_folder);
    if result.success {
        return result;
    }

    if let Some(ref err) = result.error {
        let combined = format!("{} {}", err, result.log);
        if combined.contains(".sty") && (combined.contains("not found") || combined.contains("File `")) {
            let cleaned = strip_unsupported_packages(latex_source);
            if cleaned != latex_source {
                let retry = compile_to_pdf(&cleaned, output_folder);
                if retry.success {
                    return retry;
                }
            }
        }
    }

    result
}

pub async fn compile_online(latex_source: &str, output_folder: &str) -> CompileResult {
    let cleaned_latex = strip_unsupported_packages(latex_source);

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return CompileResult {
                success: false,
                pdf_path: None,
                log: String::new(),
                error: Some(format!("Failed to create HTTP client: {}", e)),
            };
        }
    };

    let tarball = create_tarball("resume.tex", cleaned_latex.as_bytes());

    let mut last_error = String::new();
    let mut last_log = String::new();

    for host in ONLINE_HOSTS {
        let url = format!("{}/data?target=resume.tex&command=pdflatex&force=true", host);

        let part = multipart::Part::bytes(tarball.clone())
            .file_name("resume.tar")
            .mime_str("application/x-tar")
            .unwrap();
        let form = multipart::Form::new().part("file", part);

        let resp = match client.post(&url).multipart(form).send().await {
            Ok(r) => r,
            Err(e) => {
                last_error = format!("Connection to {} failed: {}", host, e);
                last_log = String::new();
                continue;
            }
        };

        let status = resp.status();
        let bytes = match resp.bytes().await {
            Ok(b) => b,
            Err(e) => {
                last_error = format!("Failed reading response from {}: {}", host, e);
                continue;
            }
        };

        if status.is_success() && bytes.len() > 100 && bytes.starts_with(b"%PDF") {
            return save_pdf(&bytes, output_folder);
        }

        let log_text = String::from_utf8_lossy(&bytes).to_string();
        if status.as_u16() == 502 || status.as_u16() == 503 || status.as_u16() == 504 {
            last_error = format!("Server {} is temporarily unavailable (HTTP {}). Trying next server...", host, status);
            last_log = log_text;
            continue;
        }

        if log_text.contains("<html") || log_text.contains("<HTML") {
            last_error = format!("Server {} returned an error page (HTTP {}). The service may be down.", host, status);
            last_log = log_text;
            continue;
        }

        return CompileResult {
            success: false,
            pdf_path: None,
            log: log_text,
            error: Some(format!(
                "Online compilation failed (HTTP {}). The LaTeX may have errors - check the log. Or try switching to Offline mode in Settings.",
                status
            )),
        };
    }

    CompileResult {
        success: false,
        pdf_path: None,
        log: last_log,
        error: Some(format!(
            "All online LaTeX servers are currently unavailable. {}. Try switching to Offline (TinyTeX) mode in Settings, or try again later.",
            last_error
        )),
    }
}

fn save_pdf(bytes: &[u8], output_folder: &str) -> CompileResult {
    let output_dir = PathBuf::from(output_folder);
    if let Err(e) = fs::create_dir_all(&output_dir) {
        return CompileResult {
            success: false,
            pdf_path: None,
            log: String::new(),
            error: Some(format!("Failed to create output folder: {}", e)),
        };
    }

    let dest_pdf = output_dir.join("Resume.pdf");
    if dest_pdf.exists() {
        fs::remove_file(&dest_pdf).ok();
    }

    if let Err(e) = fs::write(&dest_pdf, bytes) {
        return CompileResult {
            success: false,
            pdf_path: None,
            log: String::new(),
            error: Some(format!("Failed to write PDF: {}", e)),
        };
    }

    CompileResult {
        success: true,
        pdf_path: Some(dest_pdf.to_string_lossy().to_string()),
        log: "Compiled online via latexonline.cc (pdflatex)".to_string(),
        error: None,
    }
}

fn strip_unsupported_packages(latex: &str) -> String {
    let mut result = latex.to_string();

    let packages_to_remove = [
        "fontawesome5",
        "fontawesome",
        "academicons",
        "fontspec",
        "unicode-math",
        "ucharclasses",
        "polyglossia",
        "ctex",
    ];

    for pkg in &packages_to_remove {
        let patterns = [
            format!("\\usepackage{{{}}}\n", pkg),
            format!("\\usepackage{{{}}}", pkg),
            format!("\\usepackage[{{}}]{{{}}}\n", pkg),
        ];
        for pat in &patterns {
            result = result.replace(pat, "");
        }
        let re_pattern = format!("\\usepackage\\[[^\\]]*\\]{{{}}}\n?", pkg);
        if let Ok(re) = regex::Regex::new(&re_pattern) {
            result = re.replace_all(&result, "").to_string();
        }
    }

    let command_replacements = [
        ("\\faExternalLink*", " [link]"),
        ("\\faExternalLink", " [link]"),
        ("\\faGithub", "GitHub"),
        ("\\faLinkedin", "LinkedIn"),
        ("\\faLinkedinIn", "LinkedIn"),
        ("\\faEnvelope", "Email"),
        ("\\faPhone", "Phone"),
        ("\\faMapMarker*", ""),
        ("\\faMapMarker", ""),
        ("\\faGlobe", ""),
        ("\\faHome", ""),
        ("\\faBriefcase", ""),
        ("\\faGraduationCap", ""),
        ("\\faAward", ""),
        ("\\faStar", ""),
        ("\\faCode", ""),
        ("\\faServer", ""),
        ("\\faDatabase", ""),
        ("\\faPython", "Python"),
        ("\\faJs", "JS"),
        ("\\faReact", "React"),
        ("\\faNode", "Node"),
        ("\\faDocker", "Docker"),
        ("\\faAws", "AWS"),
        ("\\faLinkedinSquare", "LinkedIn"),
        ("\\faGithubSquare", "GitHub"),
        ("\\faStackOverflow", "StackOverflow"),
        ("\\faMedium", "Medium"),
        ("\\faTwitter", "Twitter"),
        ("\\faInstagram", "Instagram"),
        ("\\faYoutube", "YouTube"),
        ("\\aiOrcid", "ORCID"),
        ("\\aiGoogleScholar", "Scholar"),
        ("\\aiResearchGate", "ResearchGate"),
    ];

    for (cmd, replacement) in &command_replacements {
        result = result.replace(cmd, replacement);
    }

    if let Ok(re) = regex::Regex::new(r"\\fa[A-Z][a-zA-Z]*\*?") {
        result = re.replace_all(&result, "").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"\\ai[A-Z][a-zA-Z]*") {
        result = re.replace_all(&result, "").to_string();
    }

    result = result.replace("\\input{glyphtounicode}\n", "");
    result = result.replace("\\input{glyphtounicode}", "");
    result = result.replace("\\pdfgentounicode=1\n", "");
    result = result.replace("\\pdfgentounicode=1", "");

    result
}

fn create_tarball(filename: &str, content: &[u8]) -> Vec<u8> {
    let mut tar = Vec::new();
    let mut header = [0u8; 512];

    let name_bytes = filename.as_bytes();
    let copy_len = name_bytes.len().min(100);
    header[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

    header[100..108].copy_from_slice(b"0000644\0");
    header[108..116].copy_from_slice(b"0000000\0");
    header[116..124].copy_from_slice(b"0000000\0");

    let size_str = format!("{:011o}\0", content.len());
    header[124..136].copy_from_slice(size_str.as_bytes());

    header[136..148].copy_from_slice(b"00000000000\0");

    header[148..156].copy_from_slice(b"        ");

    header[156] = b'0';

    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");

    header[265..270].copy_from_slice(b"root\0");
    header[297..302].copy_from_slice(b"root\0");

    header[329..337].copy_from_slice(b"0000000\0");
    header[337..345].copy_from_slice(b"0000000\0");

    let checksum: u32 = header.iter().map(|&b| b as u32).sum();
    let checksum_str = format!("{:06o}\0 ", checksum);
    header[148..156].copy_from_slice(checksum_str.as_bytes());

    tar.extend_from_slice(&header);
    tar.extend_from_slice(content);

    let padding_needed = (512 - (content.len() % 512)) % 512;
    tar.extend(std::iter::repeat(0u8).take(padding_needed));

    tar.extend(std::iter::repeat(0u8).take(1024));

    tar
}
