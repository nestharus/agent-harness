pub mod contracts;
pub mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

pub fn registered_command_count() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::registered_command_count;

    #[test]
    fn phase_0a_registers_no_value_slice_commands() {
        assert_eq!(registered_command_count(), 0);
    }
}
