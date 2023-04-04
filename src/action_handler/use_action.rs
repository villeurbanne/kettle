use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    imported_files: Vec<String>,
    kettle_path: String,
}

pub fn handle_action(kettle_name: &str, destination_folder: &str, kettle_repo_path: &str) {
    if Path::new(destination_folder).is_dir() && destination_folder != "." {
        println!("\n⚠️  a folder already exists with this name");
    } else {
        let repo_kettle_vector = vec![kettle_repo_path, kettle_name];
        let repo_kettle_path = repo_kettle_vector.concat();
        if Path::new(&repo_kettle_path).exists() {
            let new_local_folder_vector = vec![destination_folder];
            let new_local_folder_path = new_local_folder_vector.concat();
            if destination_folder != "." {
                fs::create_dir(new_local_folder_path)
                    .expect("\nError encountered while creating destination folder");
            }
            let kettle_repo_recipe_vector = vec![kettle_repo_path, kettle_name, "/kettle.json"];
            let kettle_recipe = fs::read_to_string(kettle_repo_recipe_vector.concat())
                .expect("\nError encountered while reading the recipe file");

            let recipe_json: Recipe = serde_json::from_str(&kettle_recipe)
                .expect("\nError encountered while deserialising the json recipe");

            for file_name in recipe_json.imported_files {
                let repo_file_path = vec![
                    repo_kettle_path.to_owned(),
                    "/".to_string(),
                    file_name.to_owned(),
                ]
                .concat();

                let new_local_file_path =
                    vec![new_local_folder_vector.concat(), "/".to_string(), file_name].concat();

                let split_structure = new_local_file_path.split("/");

                let length_to_remove = split_structure.last().unwrap_or_default().len();
                let mut local_folder_path = new_local_file_path.clone();

                let mut i = 0;
                while i < length_to_remove {
                    local_folder_path.pop();
                    i += 1;
                }
                if !Path::new(&local_folder_path).exists() {
                    fs::create_dir_all(&local_folder_path)
                        .expect("\nError creating directory structure");
                }

                fs::copy(repo_file_path, new_local_file_path)
                    .expect("\nError encountered copying files from repo to the destination folder");
            }
            if destination_folder != "."  {
                println!("\n✅ created successfully at 📁{destination_folder}");
            } else {
                println!("\n✅ created successfully at 📁local dir");
            }
        
        } else {
            println!("\n⚠️  this kettle doesn't exist");
        }
    }
}
