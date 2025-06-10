use std::env;
use std::path::Path;
use std::fs;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "File Organizer")]
#[command(about = "Sorts files into folders by type", long_about = None)]
struct Args {
    folder: String,
    #[arg(short, long)]
    dry_run: bool,
}


fn main() {
let args = Args::parse();

let folder_path = &args.folder;

if !Path::new(folder_path).is_dir() {
    println!("The provided path is not a valid folder.");
    return;
}

println!("You selected folder: {}", folder_path);
    
    let entries = fs::read_dir(folder_path).unwrap();
    for entry_result in entries {
        let entry = entry_result.unwrap();
        let path = entry.path();

        let metadata = entry.metadata().unwrap();
        if !metadata.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let folder_name = match ext_str.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "gif" => "Images",
                    "pdf" => "PDFs",
                    "txt" | "md" => "Text",
                    "mp4" | "mov" => "Videos",
                    "zip" | "rar" => "Archives",
                    _ => "Others",
                };

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



