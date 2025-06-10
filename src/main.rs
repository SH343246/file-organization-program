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
    let name = entry.file_name();

    println!("Found {:?}", name);

}






}



