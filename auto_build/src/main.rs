use walkdir::{DirEntry, WalkDir};

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
    for entry in all_directories(dir.as_str()) {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        match path_str.find("lib") {
            is_lib_dir => println!("Found library: {}", path_str),
            None => println!("Found directory: {}", path_str),
        }
        println!("{}", entry.path().display());
    }
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn all_source_directories(root: &str) -> BuildDirInfo {
    all_directories(root)
        .map(|e| entry_to_builddirtype(e))
        .filter(Option::is_some)
        .map(Option::unwrap)
}

fn all_directories(root: &str) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_dir)
}

fn entry_to_builddirtype(entry: &DirEntry) -> Option<BuildDirInfo> {
    let path = entry.path();
    let path_str = path.to_str().unwrap();

    // prefix is the path_str up to _ or the first capital letter
    let prefix = match path_str.find(|c: char| c.is_uppercase() || c == '_') {
        Some(i) => &path_str[..i],
        None => return Option::None,
    };

    let dirtype = match prefix {
        "lib" => BuildDirType::Lib,
        "test" => BuildDirType::Test,
        "app" => BuildDirType::App,
        "prototype" => BuildDirType::Prototype,
        _ => return Option::None,
    };

    let name_after_type = &path_str[prefix.len()..];

    if name_after_type.is_empty() {
        return Option::None;
    }

    Option::Some(BuildDirInfo {
        name: name_after_type.to_owned(),
        kind: dirtype,
        path: entry.clone(),
    })
}

// BuildDirType will be used to determine what kind of directory we are in
// with a paramter to the DirEntry
enum BuildDirType {
    Lib,
    Dir,
    Test,
    App,
    Prototype,
}

struct BuildDirInfo {
    name: String,
    kind: BuildDirType,
    path: DirEntry,
}
