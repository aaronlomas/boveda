use tauri::State;
use crate::state::AppState;

/// Ejecuta un comando de texto en la CLI interna de Bóveda Core.
/// Devuelve las líneas de respuesta formateadas como strings.
#[tauri::command]
pub fn shell_query(input: String, state: State<'_, AppState>) -> Vec<String> {
    state.cmd_query_shell(&input)
}
