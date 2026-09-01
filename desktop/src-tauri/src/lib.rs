mod snapshot_command;

use snapshot_command::get_ecosystem_snapshot;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ecosystem_snapshot])
        .run(tauri::generate_context!())
        .expect("failed to run Ecosystem Observer desktop");
}
