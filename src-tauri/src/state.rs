// El AppState vive en boveda-core para ser framework-agnostico.
// Aquí solo lo re-exportamos para que lib.rs y los comandos Tauri lo encuentren.
pub use boveda_core::AppState;
