mod commands;
mod gemini;
mod history;
mod keystore;
mod latex;
mod profile;
mod templates;
mod tinytex;

use commands::AppState;
use profile::load_settings;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let settings = load_settings().unwrap_or_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState {
            settings: Mutex::new(settings),
        })
        .invoke_handler(tauri::generate_handler![
            commands::save_api_key,
            commands::load_api_key,
            commands::delete_api_key,
            commands::test_gemini_key,
            commands::get_settings,
            commands::set_output_folder,
            commands::set_model,
            commands::list_models_cmd,
            commands::set_compile_mode,
            commands::save_considerations,
            commands::reset_settings,
            commands::reset_profile,
            commands::set_import_source_manual,
            commands::save_profile_cmd,
            commands::load_profile_cmd,
            commands::import_pdf,
            commands::import_latex,
            commands::list_templates_cmd,
            commands::generate_latex,
            commands::compile_pdf,
            commands::generate_and_compile,
            commands::list_history_cmd,
            commands::delete_history_cmd,
            commands::clear_history_cmd,
            commands::tinytex_status,
            commands::install_tinytex_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TailorResume application");
}
