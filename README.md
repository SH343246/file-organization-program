# File Organizer
A rust command-line tool that organizes folder files and sub-files to folders by their extension.

# Features
- Files are organized into folders by extension type (ex. .pdf, .jpg, .pptx)
- A 'config.json' file allows for custumizable file types 
- Folders can be recursively scanned
- Testing can be mocked and actions outputted in console with '--dry-run' mode
- Built with Rust, some specific libraries include 'clap', 'serde', and 'walkdir'

# Run
cargo run -- /Users/yourname/Downloads
cargo run -- /Users/yourname/Downloads --dry-run

# Configuration File
File extensions can be added in the config.json file for greater file orginization.

# Why I Created this File
- Practice with Rust
- Learn real-world tooling such as argument parsing, file I/O, JSON parsing
