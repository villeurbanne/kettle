use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(kettle_name: &str, kettle_repo_path: &str) {
    let new_kettle_vector = vec![kettle_repo_path, kettle_name];

    let new_kettle_path = new_kettle_vector.concat();

    if !Path::new(&new_kettle_path).exists() {
        fs::create_dir(new_kettle_path).expect("Error encountered while creating kettle");

        let repo_new_file_path_vector = vec![kettle_repo_path, kettle_name, "/kettle.json"];

        let repo_new_file_path = repo_new_file_path_vector.concat();

        let mut repo_file =
            File::create(repo_new_file_path).expect("Error encountered while creating file!");

        let mut local_file =
            File::create("./kettle.json").expect("Error encountered while creating file!");

        let new_recipe = Recipe {
            name: kettle_name.to_owned(),
            imported_files: vec!["kettle.json".to_string()],
            kettle_path: repo_new_file_path_vector.concat(),
        };

        let new_recipe_json = serde_json::to_string_pretty(&new_recipe).unwrap();

        repo_file
            .write_all(new_recipe_json.as_bytes())
            .expect("Error while writing to file");

        local_file
            .write_all(new_recipe_json.as_bytes())
            .expect("Error while writing to file");

        println!("✅ kettle successfully initialised !");
    } else {
        println!("⚠️  A kettle already exists with this name")
    }
}
