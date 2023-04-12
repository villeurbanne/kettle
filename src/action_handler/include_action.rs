use serde::{Deserialize, Serialize};
use std::env::Args;
use std::fs::File;
use std::path::Path;
use std::{fs, io::Write};

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(file_name: &str, other_files: &mut Args, state: i32) {
    if Path::new(file_name).exists() {
        if Path::new(&file_name).is_dir() {
            import_dir(&file_name, other_files, state);
        } else {
            import_files(&file_name, other_files, state);
        }
    } else {
        println!("\n⚠️  This file doesn't exist");
    }
}

fn import_files(file_name: &str, other_files: &mut Args, state: i32) {
    let kettle_recipe = fs::read_to_string("kettle.json")
        .expect("\nError encountered while reading the recipe file");

    let mut recipe_json: Recipe = serde_json::from_str(&kettle_recipe)
        .expect("\nError encountered while deserialising the json recipe");
    let included_file_name = String::from(file_name);

    recipe_json.imported_files.push(included_file_name);

    let mut recipe_file = File::create("kettle.json").expect("Error while writing to file");

    let new_recipe_json = serde_json::to_string_pretty(&recipe_json).unwrap();

    recipe_file
        .write_all(new_recipe_json.as_bytes())
        .expect("\nError while writing to file");

    println!("✅ {file_name} successfully included !");

    let mut next_file = other_files.next().unwrap_or_default();
    while next_file != "" {
        let file_name = next_file.clone();
        if Path::new(&file_name).is_dir() {
            import_dir(&file_name, other_files, state);
        } else {
            let included_file_name = String::from(next_file);

            recipe_json.imported_files.push(included_file_name);

            let mut recipe_file = File::create("kettle.json").expect("Error while writing to file");

            let new_recipe_json = serde_json::to_string_pretty(&recipe_json).unwrap();

            recipe_file
                .write_all(new_recipe_json.as_bytes())
                .expect("\nError while writing to file");

            println!("\n✅ {file_name} successfully included !");
            next_file = other_files.next().unwrap_or_default();
        }
    }
}

fn import_dir(file_name: &str, other_files: &mut Args, state: i32) {
    let path = Path::new(file_name);
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        let file = entry.path().into_os_string().into_string().unwrap();
        if entry.path().is_dir() && file_name != "./.git" && state == 0 {
            import_dir(&file, other_files, state); 
        } else if file_name != "./.git" {
            import_files(&file, other_files, state);
        }
    }

}
