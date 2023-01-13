use auto_build::builddir::all_source_directories;

fn main() {
    // if no argument given, print and exit
    if std::env::args().len() < 2 {
        println!("Usage: auto_build <directory>");
        return;
    }
    // directory is first argument
    let dir = std::env::args().nth(1).expect("No directory given");

    // if dir is not a directory, stop
    if !std::path::Path::new(&dir).is_dir() {
        println!("{} is not a directory", dir);
        return;
    }

    println!("Building {}...", dir);

    for source_dir in all_source_directories(&dir) {
        println!(
            "{} {:?} {}",
            source_dir.name,
            source_dir.kind,
            source_dir.entry.path().display()
        );
    }
}
