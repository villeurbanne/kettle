use std::{fs, path::Path};

pub fn handle_action(kettle_name: &str, kettle_repo_path: &str) {
    let kettle_path = vec![kettle_repo_path, kettle_name].concat();

    if Path::new(&kettle_path).exists() {
        let remove = fs::remove_dir_all(kettle_path);
        match remove {
            Ok(_) => println!("✅ {kettle_name} was successfully deleted !"),
            Err(e) => println!("error deleting the kettle {e}"),
        }
    } else {
        println!("⚠️  This kettle doesn't exist")
    }
}
