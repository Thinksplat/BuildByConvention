use std::vec;

use auto_build::builddir::all_exec_directories;
use auto_build::builddir::all_lib_directories;
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
    let project_root = std::path::Path::new(&dir);

    // if dir is not a directory, stop
    if !project_root.is_dir() {
        println!("{} is not a directory", dir);
        return;
    }

    println!("Building {}...", dir);

    let mut lib_data = vec![];
    let mut all_dirs = vec![];
    println!("Generating lib dirs...");
    for lib_dir in all_lib_directories(&dir) {
        if let BuildDirType::Lib(info) = lib_dir {
            let libdata = generators::generate_lib(&info);
            lib_data.push(libdata);
            all_dirs.push(info.path().to_owned());
        }
    }

    println!("Generating app,prototype,test dirs...");
    for exec_dir in all_exec_directories(&dir) {
        match exec_dir {
            BuildDirType::App(info) => {
                generators::generate_app(project_root, &info, &lib_data);
                all_dirs.push(info.path().to_owned());
            }
            BuildDirType::Test(info) => {
                generators::generate_test(project_root, &info, &lib_data);
                all_dirs.push(info.path().to_owned());
            }
            BuildDirType::Prototype(info) => {
                generators::generate_prototype(project_root, &info, &lib_data);
                all_dirs.push(info.path().to_owned());
            }
            _ => (),
        }
    }

    generators::generate_main(project_root, all_dirs.as_slice());

    // Using the lib_dirs, generate the app, test, and prototype directories
    //for exec_dir in all_exec_directories()

    //     let path_relative_to_dir = entry.path().strip_prefix(&dir).unwrap();
    //     let path_str = path_relative_to_dir.to_str().unwrap();
    //     list_dirs.push(path_str.to_owned());

    //     match source_dir.kind {
    //         BuildDirType::Lib => {
    //             generators::generate_lib(&source_dir);
    //         }
    //         BuildDirType::App => (),
    //         BuildDirType::Test => {
    //             generators::generate_test(entry);
    //         }
    //         BuildDirType::Prototype => {
    //             generators::generate_prototype(entry);
    //         }
    //     }
    // }
    // generators::generate_main(&dirpath, &list_dirs);
}
