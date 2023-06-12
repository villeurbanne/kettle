mod action_handler;
use std::fs;
use std::path::Path;

use action_handler::handle_action;

fn main() {
    let mut args = std::env::args();
    let kettle_repo_path ="~/.config/kettle/";
    if !Path::new(&kettle_repo_path).exists() {
        fs::create_dir_all(&kettle_repo_path).expect("Error creating directory structure");
    }
    handle_action(&mut args, &kettle_repo_path);

}
