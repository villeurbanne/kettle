use std::process::Command;

pub fn handle_action() {
    Command::new("kettlecli")
        .status()
        .expect("Failed to run kettle cli !");
}
