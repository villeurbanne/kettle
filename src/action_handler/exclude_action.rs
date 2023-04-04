use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::{fs, io::Write};

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(file_name: &str) {
    if Path::new(file_name).exists() {
        let kettle_recipe = fs::read_to_string("kettle.json")
            .expect("\nError encountered while reading the recipe file");

        let mut recipe_json: Recipe = serde_json::from_str(&kettle_recipe)
            .expect("\nError encountered while deserialising the json recipe");

        let included_file_name = String::from(file_name);
        if recipe_json.imported_files.contains(&included_file_name) {
            let file_index = recipe_json
                .imported_files
                .iter()
                .position(|x| x == file_name)
                .unwrap();

            recipe_json.imported_files.remove(file_index);

            let mut recipe_file = File::create("kettle.json").expect("Error while writing to file");
            let new_recipe_json = serde_json::to_string_pretty(&recipe_json).unwrap();

            recipe_file
                .write_all(new_recipe_json.as_bytes())
                .expect("\nError while writing to file");
        } else {
            println!("\nThe file was already not in the kettle.json")
        }
        println!("\n✅ {file_name} successfully excluded !");
    } else {
        println!("\n⚠️  This file doesn't exist");
    }
}
