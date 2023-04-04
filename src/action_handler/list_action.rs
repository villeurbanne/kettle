use std::fs::{self, read_dir};
use std::path::Path;

pub fn handle_action(kettle_repo_path: &str, state: i32) {
    if Path::new(kettle_repo_path).exists() {
        match state {
            0 => normal_output(kettle_repo_path),
            1 => plain_output(kettle_repo_path),
            _ => println!("⚠️  Invalid state"),
        }
    }
}

pub fn normal_output(kettle_repo_path: &str) {
    let mut kettles_count = read_dir(kettle_repo_path).unwrap().count();

    println!("\n💌 You have {} kettle(s) :\n", kettles_count);
    for file in fs::read_dir(kettle_repo_path).unwrap() {
        let unwraped_file = file.unwrap().path();

        if Path::new(&unwraped_file.as_path()).is_dir() {
            let kettle = unwraped_file.to_str().unwrap().split('/');
            println!(" -> {}", kettle.last().unwrap());
        } else {
            kettles_count -= 1;
        }
    }
    println!("");
}

fn plain_output(kettle_repo_path: &str) {
    for file in fs::read_dir(kettle_repo_path).unwrap() {
        let unwraped_file = file.unwrap().path();

        if Path::new(&unwraped_file.as_path()).is_dir() {
            let kettle = unwraped_file.to_str().unwrap().split('/');
            println!("{}", kettle.last().unwrap());
        }
    }
}

