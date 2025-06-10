use std::env;
use std::path::Path;
use std::fs;





fn main() {
let args: Vec<String> = env::args().collect();

if args.len() < 2 {
    println!("Usage: file-organizer <folder-path>");
    return;
}

let folder_path = &args[1];

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
                fs::create_dir_all(&new_folder_path).unwrap();

                let file_name = entry.file_name();
                let new_path = format!("{}/{}", new_folder_path, file_name.to_string_lossy());
                fs::rename(&path, new_path).unwrap();
            }
        }
    }
}



