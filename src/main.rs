use std::env;
use std::path::Path;
use std::fs;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json;
use walkdir::WalkDir;


#[derive(Parser, Debug)]
#[command(name = "File Organizer")]
#[command(about = "Sorts files into folders by type", long_about = None)]
struct Args {
    folder: String,
    #[arg(short, long)]
    dry_run: bool,
}

fn load_config(path: &str) -> HashMap<String, String> {
    let file = File::open(path).expect("Could not open config file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Error trying to read tjhe file")
}

fn main() {

let config = load_config("config.json");

let args = Args::parse();

let folder_path = &args.folder;

if !Path::new(folder_path).is_dir() {
    println!("The path is not a valid folder.");
    return;
}

println!("You selected folder: {}", folder_path);
    
    for entry_result in WalkDir::new(folder_path) {
    let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };      
        let path = entry.path();

         if !path.is_file() {
            continue;
        }


        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let folder_name = config
                    .get(&ext_str.to_lowercase())
                    .map(|s| s.as_str())
                    .unwrap_or("Others");
                let new_folder_path = format!("{}/{}", folder_path, folder_name);

                let file_name = entry.file_name();
                let new_path = format!("{}/{}", new_folder_path, file_name.to_string_lossy());
                if args.dry_run {
                println!("[DRY RUN] Would move {:?} to: {}", file_name, new_path);
                } else {
                fs::create_dir_all(&new_folder_path).unwrap();
                fs::rename(&path, &new_path).unwrap();
                println!("Moved {:?} to: {}", file_name, new_path);
}

            }
        }
    }
}



