use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Basics {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkExperience {
    pub id: String,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tech: Vec<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillGroup {
    pub id: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    pub id: String,
    #[serde(default)]
    pub institution: String,
    #[serde(default)]
    pub degree: String,
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub gpa: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certification {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub awarder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub id: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(default)]
    pub basics: Basics,
    #[serde(default)]
    pub work: Vec<WorkExperience>,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub skills: Vec<SkillGroup>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub awards: Vec<Award>,
    #[serde(default)]
    pub custom: Vec<CustomField>,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            basics: Basics {
                name: String::new(),
                email: String::new(),
                phone: String::new(),
                location: String::new(),
                website: String::new(),
                github: String::new(),
                linkedin: String::new(),
                summary: String::new(),
            },
            work: Vec::new(),
            projects: Vec::new(),
            skills: Vec::new(),
            education: Vec::new(),
            certifications: Vec::new(),
            awards: Vec::new(),
            custom: Vec::new(),
        }
    }
}

pub fn app_config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("tailor-resume")
}

pub fn profile_path() -> PathBuf {
    app_config_dir().join("profile.json")
}

pub fn settings_path() -> PathBuf {
    app_config_dir().join("settings.json")
}

pub fn save_profile(profile: &Profile) -> Result<(), String> {
    let dir = app_config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
    let json = serde_json::to_string_pretty(profile).map_err(|e| format!("Serialize error: {}", e))?;
    fs::write(profile_path(), json).map_err(|e| format!("Write error: {}", e))?;
    Ok(())
}

pub fn load_profile() -> Result<Profile, String> {
    let path = profile_path();
    if !path.exists() {
        return Ok(Profile::default());
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))?;
    let profile: Profile = serde_json::from_str(&json).map_err(|e| format!("Deserialize error: {}", e))?;
    Ok(profile)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub output_folder: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub tinytex_path: String,
    #[serde(default = "default_compile_mode")]
    pub compile_mode: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub considerations: Vec<String>,
    #[serde(default)]
    pub source_latex: String,
    #[serde(default)]
    pub import_source: String,
    #[serde(default = "default_extract_prompt_pdf")]
    pub extract_prompt_pdf: String,
    #[serde(default = "default_extract_prompt_latex")]
    pub extract_prompt_latex: String,
    #[serde(default = "default_generate_prompt")]
    pub generate_prompt: String,
}

fn default_model() -> String {
    "gemini-2.5-flash".to_string()
}

fn default_compile_mode() -> String {
    "online".to_string()
}

pub fn default_extract_prompt_pdf() -> String {
    "Extract a structured resume from this PDF. Return a JSON object with exactly this structure:\n\
    {\n\
      \"basics\": {\"name\":\"\",\"email\":\"\",\"phone\":\"\",\"location\":\"\",\"website\":\"\",\"github\":\"\",\"linkedin\":\"\",\"summary\":\"\"},\n\
      \"work\": [{\"id\":\"\",\"company\":\"\",\"position\":\"\",\"startDate\":\"\",\"endDate\":\"\",\"highlights\":[]}],\n\
      \"projects\": [{\"id\":\"\",\"name\":\"\",\"description\":\"\",\"tech\":[],\"url\":\"\",\"highlights\":[]}],\n\
      \"skills\": [{\"id\":\"\",\"category\":\"\",\"items\":[]}],\n\
      \"education\": [{\"id\":\"\",\"institution\":\"\",\"degree\":\"\",\"field\":\"\",\"startDate\":\"\",\"endDate\":\"\",\"gpa\":\"\",\"highlights\":[]}],\n\
      \"certifications\": [{\"id\":\"\",\"name\":\"\",\"issuer\":\"\",\"date\":\"\"}],\n\
      \"awards\": [{\"id\":\"\",\"title\":\"\",\"date\":\"\",\"awarder\":\"\"}],\n\
      \"custom\": [{\"id\":\"\",\"key\":\"\",\"value\":\"\"}]\n\
    }\n\
    Generate a UUID for each id field. Fill all fields from the resume content. Use empty strings for missing data. Return ONLY the JSON.".to_string()
}

pub fn default_extract_prompt_latex() -> String {
    "Extract a structured resume from this LaTeX source. Return a JSON object with exactly this structure:\n\
    {\n\
      \"basics\": {\"name\":\"\",\"email\":\"\",\"phone\":\"\",\"location\":\"\",\"website\":\"\",\"github\":\"\",\"linkedin\":\"\",\"summary\":\"\"},\n\
      \"work\": [{\"id\":\"\",\"company\":\"\",\"position\":\"\",\"startDate\":\"\",\"endDate\":\"\",\"highlights\":[]}],\n\
      \"projects\": [{\"id\":\"\",\"name\":\"\",\"description\":\"\",\"tech\":[],\"url\":\"\",\"highlights\":[]}],\n\
      \"skills\": [{\"id\":\"\",\"category\":\"\",\"items\":[]}],\n\
      \"education\": [{\"id\":\"\",\"institution\":\"\",\"degree\":\"\",\"field\":\"\",\"startDate\":\"\",\"endDate\":\"\",\"gpa\":\"\",\"highlights\":[]}],\n\
      \"certifications\": [{\"id\":\"\",\"name\":\"\",\"issuer\":\"\",\"date\":\"\"}],\n\
      \"awards\": [{\"id\":\"\",\"title\":\"\",\"date\":\"\",\"awarder\":\"\"}],\n\
      \"custom\": [{\"id\":\"\",\"key\":\"\",\"value\":\"\"}]\n\
    }\n\
    Generate a UUID for each id field. Fill all fields from the resume content. Use empty strings for missing data. Return ONLY the JSON.".to_string()
}

pub fn default_generate_prompt() -> String {
    "You are an expert resume writer. Create a complete, compilable pdfLaTeX document for a resume tailored to a job description.\n\n\
    ## Candidate Profile (JSON):\n{PROFILE}\n\n\
    ## Job Description:\n{JD}\n\n\
    ## Considerations (must address these):\n{CONSIDERATIONS}\n\n\
    ## Template Structure (use this as the base, fill in tailored content):\n{TEMPLATE}\n\n\
    ## Instructions:\n\
    1. Tailor the summary, work highlights, project descriptions, and skills to match the job description.\n\
    2. Quantify achievements where possible. Use action verbs.\n\
    3. Reorder and emphasize experiences most relevant to the JD.\n\
    4. Keep it to one page if possible, maximum two pages.\n\
    5. Ensure the LaTeX compiles with pdfLaTeX (not XeLaTeX).\n\
    6. Output ONLY the complete .tex source code. No explanations, no markdown fences.\n\
    7. Use the template structure above but customize all content fields for the JD.\n\
    8. Keep the same LaTeX formatting, packages, and structure as the template.".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        let docs = dirs::document_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
        Settings {
            output_folder: docs.join("TailorResume").to_string_lossy().to_string(),
            model: default_model(),
            tinytex_path: String::new(),
            compile_mode: default_compile_mode(),
            api_key: String::new(),
            considerations: Vec::new(),
            source_latex: String::new(),
            import_source: String::new(),
            extract_prompt_pdf: default_extract_prompt_pdf(),
            extract_prompt_latex: default_extract_prompt_latex(),
            generate_prompt: default_generate_prompt(),
        }
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let dir = app_config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
    let json = serde_json::to_string_pretty(settings).map_err(|e| format!("Serialize error: {}", e))?;
    fs::write(settings_path(), json).map_err(|e| format!("Write error: {}", e))?;
    Ok(())
}

pub fn load_settings() -> Result<Settings, String> {
    let path = settings_path();
    if !path.exists() {
        let defaults = Settings::default();
        save_settings(&defaults)?;
        return Ok(defaults);
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))?;
    let settings: Settings = serde_json::from_str(&json).map_err(|e| format!("Deserialize error: {}", e))?;
    Ok(settings)
}
