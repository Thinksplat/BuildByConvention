use std::vec;

use auto_build::builddir::all_source_directories;
use auto_build::builddir::BuildDirType;
use auto_build::generators;

fn main() {
    // if no argument given, print and exit
    if std::env::args().len() < 2 {
        println!("Usage: auto_build <directory>");
        return;
    }
    // directory is first argument
    let dir = std::env::args().nth(1).expect("No directory given");
    let dirpath = std::path::Path::new(&dir);

    // if dir is not a directory, stop
    if !dirpath.is_dir() {
        println!("{} is not a directory", dir);
        return;
    }

    println!("Building {}...", dir);

    let mut list_dirs = vec![];
    println!("Generating lib dirs...");
    for lib_dir in all_source_directories(&dir).filter(|d| d.kind == BuildDirType::Lib) {
        println!(
            "{} {:?} {}",
            source_dir.name,
            source_dir.kind,
            source_dir.entry.path().display()
        );
        let entry = &source_dir.entry;
        generators::generate_lib(&lib_dir);

        let path_relative_to_dir = entry.path().strip_prefix(&dir).unwrap();
        let path_str = path_relative_to_dir.to_str().unwrap();
        list_dirs.push(path_str.to_owned());

        match source_dir.kind {
            BuildDirType::Lib => {
                generators::generate_lib(&source_dir);
            }
            BuildDirType::App => (),
            BuildDirType::Test => {
                generators::generate_test(entry);
            }
            BuildDirType::Prototype => {
                generators::generate_prototype(entry);
            }
        }
    }
    generators::generate_main(&dirpath, &list_dirs);
}
