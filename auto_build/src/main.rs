use walkdir::{WalkDir, DirEntry};

fn main() {
    // if no argument given, print and exit
    if std::env::args().len() < 2 {
        println!("Usage: auto_build <directory>");
        return;
    }
    // directory is first argument
    let dir = std::env::args().nth(1).expect("No directory given");
    println!("Building {}...", dir);

    println!("Searching for libraries...");
    for entry in lib_directories(dir.as_str())
    {
        println!("{}", entry.path().display());
    }
}

fn is_lib_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_str().unwrap().starts_with("lib")
}

fn lib_directories(root: &str) -> impl Iterator<Item = DirEntry> 
{
    WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(is_lib_dir)
}
