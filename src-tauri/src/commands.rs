use tauri::State;
use std::sync::Mutex;

use crate::gemini::GeminiClient;
use crate::history::{HistoryEntry, add_history, load_history, delete_history as del_history, clear_history as clr_history};
use crate::keystore;
use crate::latex::{CompileResult, try_compile_offline, compile_online};
use crate::profile::{Profile, Settings, load_profile, save_profile, save_settings};
use crate::templates::{Template, list_templates, get_template_source};
use crate::tinytex;
use crate::gemini::{ModelInfo, list_available_models};

pub struct AppState {
    pub settings: Mutex<Settings>,
}

fn get_api_key(state: &State<AppState>) -> Result<String, String> {
    if let Ok(key) = keystore::load_api_key() {
        if !key.is_empty() {
            return Ok(key);
        }
    }
    let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(settings.api_key.clone())
}

#[tauri::command]
pub fn save_api_key(key: String, state: State<AppState>) -> Result<(), String> {
    let _ = keystore::save_api_key(&key);
    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.api_key = key;
        save_settings(&settings)?;
    }
    Ok(())
}

#[tauri::command]
pub fn load_api_key(state: State<AppState>) -> Result<String, String> {
    get_api_key(&state)
}

#[tauri::command]
pub fn delete_api_key(state: State<AppState>) -> Result<(), String> {
    let _ = keystore::delete_api_key();
    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.api_key = String::new();
        save_settings(&settings)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn test_gemini_key(key: String, model: String) -> Result<bool, String> {
    let client = GeminiClient::new(&key, &model);
    client.test_connection().await
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(settings.clone())
}

#[tauri::command]
pub fn set_output_folder(folder: String, state: State<AppState>) -> Result<(), String> {
    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.output_folder = folder.clone();
    }
    let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    save_settings(&settings)
}

#[tauri::command]
pub fn set_compile_mode(mode: String, state: State<AppState>) -> Result<(), String> {
    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.compile_mode = mode;
    }
    let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    save_settings(&settings)
}

#[tauri::command]
pub fn save_considerations(considerations: Vec<String>, state: State<AppState>) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    settings.considerations = considerations;
    save_settings(&settings)
}

#[tauri::command]
pub fn reset_settings(state: State<AppState>) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    let api_key = settings.api_key.clone();
    *settings = Settings::default();
    settings.api_key = api_key;
    save_settings(&settings)
}

#[tauri::command]
pub fn reset_profile() -> Result<(), String> {
    save_profile(&Profile::default())
}

#[tauri::command]
pub fn set_import_source_manual(state: State<AppState>) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    settings.import_source = "manual".to_string();
    settings.source_latex = String::new();
    save_settings(&settings)
}

#[tauri::command]
pub fn save_profile_cmd(profile: Profile) -> Result<(), String> {
    save_profile(&profile)
}

#[tauri::command]
pub fn load_profile_cmd() -> Result<Profile, String> {
    load_profile()
}

#[tauri::command]
pub async fn import_pdf(file_path: String, state: State<'_, AppState>) -> Result<Profile, String> {
    let api_key = get_api_key(&state)?;
    let (model, prompt) = {
        let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        (settings.model.clone(), settings.extract_prompt_pdf.clone())
    };

    if api_key.is_empty() {
        return Err("No API key set. Go to Settings to add your Google AI Studio API key.".to_string());
    }

    let client = GeminiClient::new(&api_key, &model);
    let profile = client.extract_profile_from_pdf(&file_path, &prompt).await?;

    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.import_source = "pdf".to_string();
        settings.source_latex = String::new();
        save_settings(&settings)?;
    }

    Ok(profile)
}

#[tauri::command]
pub fn import_latex(latex: String, state: State<AppState>) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    settings.import_source = "latex".to_string();
    settings.source_latex = latex;
    save_settings(&settings)?;
    Ok(())
}

#[tauri::command]
pub fn list_templates_cmd() -> Result<Vec<Template>, String> {
    Ok(list_templates())
}

#[tauri::command]
pub fn list_models_cmd() -> Result<Vec<ModelInfo>, String> {
    Ok(list_available_models())
}

#[tauri::command]
pub fn set_model(model: String, state: State<AppState>) -> Result<(), String> {
    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.model = model;
    }
    let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
    save_settings(&settings)
}

#[tauri::command]
pub async fn generate_latex(
    jd: String,
    considerations: Vec<String>,
    template_id: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let api_key = get_api_key(&state)?;
    let (model, source_latex) = {
        let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        (settings.model.clone(), settings.source_latex.clone())
    };

    if api_key.is_empty() {
        return Err("No API key set. Go to Settings to add your Google AI Studio API key.".to_string());
    }

    let client = GeminiClient::new(&api_key, &model);

    if template_id == "my-latex" {
        if source_latex.is_empty() {
            return Err("No imported LaTeX found. Import a LaTeX resume first, or select a built-in template.".to_string());
        }
        client.generate_latex_from_source(&source_latex, &jd, &considerations).await
    } else {
        let profile = load_profile()?;
        let template_source = get_template_source(&template_id)
            .ok_or("Template not found".to_string())?;
        client.generate_latex_from_profile(&profile, &jd, &considerations, &template_source).await
    }
}

#[tauri::command]
pub async fn compile_pdf(latex_source: String, state: State<'_, AppState>) -> Result<CompileResult, String> {
    let (mode, output_folder) = {
        let settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        (settings.compile_mode.clone(), settings.output_folder.clone())
    };
    if mode == "online" {
        Ok(compile_online(&latex_source, &output_folder).await)
    } else {
        Ok(try_compile_offline(&latex_source, &output_folder))
    }
}

#[tauri::command]
pub async fn generate_and_compile(
    jd: String,
    considerations: Vec<String>,
    template_id: String,
    template_name: String,
    state: State<'_, AppState>,
) -> Result<CompileResult, String> {
    let latex = generate_latex(jd.clone(), considerations.clone(), template_id.clone(), state.clone()).await?;
    let (mode, output_folder) = {
        let s = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        (s.compile_mode.clone(), s.output_folder.clone())
    };
    let result = if mode == "online" {
        compile_online(&latex, &output_folder).await
    } else {
        try_compile_offline(&latex, &output_folder)
    };

    let _ = add_history(
        &jd,
        &template_name,
        &latex,
        result.pdf_path.as_deref(),
    );

    Ok(result)
}

#[tauri::command]
pub fn list_history_cmd() -> Result<Vec<HistoryEntry>, String> {
    Ok(load_history())
}

#[tauri::command]
pub fn delete_history_cmd(id: String) -> Result<(), String> {
    del_history(&id)
}

#[tauri::command]
pub fn clear_history_cmd() -> Result<(), String> {
    clr_history()
}

#[tauri::command]
pub fn tinytex_status() -> Result<bool, String> {
    Ok(tinytex::tinytex_installed())
}

#[tauri::command]
pub async fn install_tinytex_cmd(state: State<'_, AppState>) -> Result<String, String> {
    let pdflatex_path = tinytex::install_tinytex().await?;
    tinytex::install_packages(&pdflatex_path)?;

    {
        let mut settings = state.settings.lock().map_err(|e| format!("Lock error: {}", e))?;
        settings.tinytex_path = pdflatex_path.to_string_lossy().to_string();
        save_settings(&settings)?;
    }

    Ok(pdflatex_path.to_string_lossy().to_string())
}
