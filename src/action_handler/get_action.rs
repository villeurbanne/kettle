use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use reqwest::blocking::Client;
use zip::ZipArchive;

fn get_kettle(client: &Client, kettle: &str) -> Result<String, Box<dyn std::error::Error>>
{
    let kettle = kettle.to_owned();
    let mut parts = kettle.split('@');
    let author = parts.next().unwrap();
    let name = parts.next().unwrap();

    // Send a GET request to /marketplace/kettle
    let response = client
        .get(format!("https://huqgbqslpowxjqxbphux.supabase.co/storage/v1/object/public/kettles/{}/{}.zip", author, name))
        .send()?
        .text()?;

    // Check if the response contains an error
    if response.contains("error") {
        return Err(response.into());
    }

    let url = format!("https://huqgbqslpowxjqxbphux.supabase.co/storage/v1/object/public/kettles/{}/{}.zip", author, name);
    Ok(url)
}

fn download_kettle(url: &str, path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let client = Client::new();

    // Send a GET request to the URL returned by the first request
    let mut response = client.get(url).send()?;

    // Create a new directory at the specified path
    fs::create_dir_all(path)?;
    println!("🫖 successfully installed '{}' kettle !\n", name);

    let newpath = path.join(name);
    let mut file = File::create(&newpath)?;

    // Copy the response body to the file
    copy(&mut response, &mut file)?;

    // Unzip the file at the specified path
    let mut archive = ZipArchive::new(File::open(&newpath)?)?;
    archive.extract(&newpath.parent().unwrap())?;

    // Remove the zip file
    fs::remove_file(&newpath)?;

    Ok(())
}

pub fn handle_action(kettle: &str, path: &str) -> Result<(), Box<dyn std::error::Error>>
{
    // Check if the kettle name is valid
    if (kettle.contains('@')) == false {
        println!("⚠️  error : kettle name must be in the form 'author@kettle'\n");
        return Err("".into());
    }

    let parts: Vec<&str> = kettle.split('@').collect();
    let name = parts[1];
    let location = Path::new(path).join(name);

    let client = Client::new();
    let url = match get_kettle(&client, kettle) {
        Ok(url) => url,
        Err(err) => {
             println!("⚠️  error : '{}' kettle not in marketplace\n", name);
            return Err(err);
        }
    };

    if let Err(err) = download_kettle(&url, &location, name) {
        println!("⚠️ error : download failed for '{}' kettle", name);
        return Err(err);
    }

    Ok(())
}
