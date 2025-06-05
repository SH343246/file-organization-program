use std::env;
use std::path;





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


}


