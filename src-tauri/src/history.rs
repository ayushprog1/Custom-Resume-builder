use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use chrono::Utc;

use crate::profile::app_config_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub jd_snippet: String,
    pub template_name: String,
    pub latex: String,
    pub pdf_path: Option<String>,
}

pub fn history_path() -> std::path::PathBuf {
    app_config_dir().join("history.json")
}

pub fn load_history() -> Vec<HistoryEntry> {
    let path = history_path();
    if !path.exists() {
        return Vec::new();
    }
    match fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_history(history: &[HistoryEntry]) -> Result<(), String> {
    let dir = app_config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Create dir error: {}", e))?;
    let json = serde_json::to_string_pretty(history).map_err(|e| format!("Serialize error: {}", e))?;
    fs::write(history_path(), json).map_err(|e| format!("Write error: {}", e))?;
    Ok(())
}

pub fn add_history(jd: &str, template_name: &str, latex: &str, pdf_path: Option<&str>) -> Result<HistoryEntry, String> {
    let mut history = load_history();
    let entry = HistoryEntry {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        jd_snippet: jd.chars().take(200).collect(),
        template_name: template_name.to_string(),
        latex: latex.to_string(),
        pdf_path: pdf_path.map(|p| p.to_string()),
    };
    history.insert(0, entry.clone());
    if history.len() > 50 {
        history.truncate(50);
    }
    save_history(&history)?;
    Ok(entry)
}

pub fn delete_history(id: &str) -> Result<(), String> {
    let mut history = load_history();
    history.retain(|e| e.id != id);
    save_history(&history)
}

pub fn clear_history() -> Result<(), String> {
    save_history(&[])
}
