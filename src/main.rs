mod action_handler;
use std::fs;
use std::path::Path;
extern crate dirs;

use action_handler::handle_action;

fn main() {
    let mut args = std::env::args();

    match dirs::home_dir() {
        Some(path) => {
            let home_dir: &str = &path.display().to_string();
            let kettle_repo_path = vec![home_dir, "/.config/kettle/"].concat();
            if !Path::new(&kettle_repo_path).exists() {
                fs::create_dir_all(&kettle_repo_path).expect("Error creating directory structure");
            }
            handle_action(&mut args, &kettle_repo_path);
        }
        None => println!("Impossible to get your home dir!"),
    }
}
