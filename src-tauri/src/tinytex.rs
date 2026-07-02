use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::profile::app_config_dir;

#[cfg(target_os = "windows")]
const TINYTEX_URL: &str = "https://yihui.org/tinytex/TinyTeX.zip";
#[cfg(not(target_os = "windows"))]
const TINYTEX_URL: &str = "https://yihui.org/tinytex/TinyTeX.tgz";

pub fn tinytex_dir() -> PathBuf {
    app_config_dir().join("TinyTeX")
}

pub fn tinytex_installed() -> bool {
    find_pdflatex().is_some()
}

pub fn find_pdflatex() -> Option<PathBuf> {
    let tex_dir = tinytex_dir();
    if tex_dir.exists() {
        #[cfg(target_os = "windows")]
        {
            let path = tex_dir.join("bin").join("windows");
            if path.exists() {
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.file_name().and_then(|n| n.to_str()) == Some("pdflatex.exe") {
                            return Some(p);
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            let path = tex_dir.join("bin").join("universal-darwin");
            if path.join("pdflatex").exists() {
                return Some(path.join("pdflatex"));
            }
        }
        #[cfg(target_os = "linux")]
        {
            let path = tex_dir.join("bin").join("x86_64-linux");
            if path.join("pdflatex").exists() {
                return Some(path.join("pdflatex"));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = Command::new("where").args(["pdflatex"]).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = text.lines().next() {
                    return Some(PathBuf::from(first_line.trim()));
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(output) = Command::new("which").args(["pdflatex"]).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                return Some(PathBuf::from(text.trim()));
            }
        }
    }

    None
}

pub async fn install_tinytex() -> Result<PathBuf, String> {
    let target_dir = tinytex_dir();
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("Failed to remove old TinyTeX: {}", e))?;
    }

    let temp_dir = app_config_dir().join("_tinytex_download");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let archive_name = if cfg!(target_os = "windows") { "TinyTeX.zip" } else { "TinyTeX.tgz" };
    let archive_path = temp_dir.join(archive_name);

    log::info!("Downloading TinyTeX from {}", TINYTEX_URL);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let resp = client
        .get(TINYTEX_URL)
        .send()
        .await
        .map_err(|e| format!("Download error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Download failed with status: {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Download read error: {}", e))?;

    fs::write(&archive_path, &bytes)
        .map_err(|e| format!("Failed to save archive: {}", e))?;

    log::info!("Extracting TinyTeX to {:?}", target_dir);
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create TinyTeX dir: {}", e))?;

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    archive_path.to_string_lossy(),
                    target_dir.to_string_lossy()
                ),
            ])
            .output()
            .map_err(|e| format!("Extract error: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Extraction failed: {}", err));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("tar")
            .args([
                "-xzf",
                archive_path.to_str().unwrap(),
                "-C",
                target_dir.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("Extract error: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Extraction failed: {}", err));
        }
    }

    fs::remove_dir_all(&temp_dir).ok();

    let pdflatex = find_pdflatex()
        .ok_or("TinyTeX installed but pdflatex not found")?;

    log::info!("TinyTeX installed at {:?}", pdflatex);
    Ok(pdflatex)
}

pub fn install_packages(pdflatex_path: &PathBuf) -> Result<(), String> {
    let bin_dir = pdflatex_path.parent().ok_or("Cannot find bin dir")?;
    let tlmgr = if cfg!(target_os = "windows") {
        bin_dir.join("tlmgr.bat")
    } else {
        bin_dir.join("tlmgr")
    };

    if !tlmgr.exists() {
        return Err("tlmgr not found in TinyTeX".to_string());
    }

    let packages = [
        "xcolor",
        "enumitem",
        "hyperref",
        "geometry",
        "parskip",
        "lmodern",
        "lm-math",
        "latex-bin",
        "collection-latex",
        "collection-latexrecommended",
        "fontawesome5",
        "collection-fontsrecommended",
    ];

    for pkg in &packages {
        let output = Command::new(&tlmgr)
            .args(["install", pkg])
            .output()
            .map_err(|e| format!("Failed to run tlmgr install {}: {}", pkg, e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            log::warn!("Failed to install package {}: {}", pkg, err);
        }
    }

    Ok(())
}
