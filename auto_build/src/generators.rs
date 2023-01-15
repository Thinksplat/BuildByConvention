use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs;
use std::path;

use crate::builddir::DirInfo;

fn write_template2(
    template_file: &path::Path,
    data: BTreeMap<&str, &[String]>,
    str_replace: &[(&str, &str)],
    path_to_write: &path::Path,
) {
    let mut template_contents =
        fs::read_to_string(template_file).expect("Could not read template file");

    // DO the str replaces first
    for (from, to) in str_replace {
        template_contents = template_contents.replace(from, to);
    }

    // create the handlebars registry
    let mut handlebars = Handlebars::new();
    // register the template. The template string will be verified and compiled.
    const TEMPLATE_NAME: &str = "template";
    handlebars
        .register_template_string(TEMPLATE_NAME, template_contents)
        .expect("Could not compile template");
    let result = handlebars
        .render(TEMPLATE_NAME, &data)
        .expect("Could not render template");

    println!("Writing to {:?}", path_to_write);

    fs::write(path_to_write, result).expect("Could not write CMakeLists.txt");
}

pub struct LibData {
    pub name: String,
    pub path: std::path::PathBuf,
}

pub fn generate_lib(dir: &DirInfo) -> LibData {
    let template_file = path::Path::new("templates/lib.CMakeLists");
    let out_file = dir.join("CMakeLists.txt");

    let replace = vec![("LIB_NAME", dir.name.as_str())];

    write_template2(
        template_file,
        BTreeMap::new(),
        replace.as_slice(),
        &out_file,
    );
    LibData {
        name: dir.name.to_string(),
        path: dir.path().into(),
    }
}

pub fn generate_app(project_root: &path::Path, base: &DirInfo, lib_dirs: &[LibData]) {
    generate_exec(project_root, base, lib_dirs, "app");
}

fn generate_exec(
    project_root: &path::Path,
    base: &DirInfo,
    lib_dirs: &[LibData],
    template_prefix: &str,
) {
    // By default, we include all libs

    let mut data = BTreeMap::new();

    let libs = lib_dirs
        .iter()
        .map(|l| l.name.to_owned())
        .collect::<Vec<_>>();

    data.insert("LIBS", libs.as_slice());

    let include_dirs = lib_dirs
        .iter()
        .map(|l| path_relative_to_dir(project_root, l.path.as_path()))
        .collect::<Vec<_>>();
    data.insert("INCLUDE_DIRS", include_dirs.as_slice());

    let app_name = base.entry.path().file_name().unwrap().to_str().unwrap();
    let replace = vec![("EXEC_NAME", app_name)];

    let template_file = format!("templates/{}.CMakeLists", template_prefix);
    let template_file = path::Path::new(&template_file);
    let path_to_write = base.join("CMakeLists.txt");

    write_template2(template_file, data, &replace, path_to_write.as_path());
}

pub fn generate_prototype(project_root: &path::Path, base: &DirInfo, lib_dirs: &[LibData]) {
    generate_exec(project_root, base, lib_dirs, "prototype");
}

pub fn generate_test(project_root: &path::Path, base: &DirInfo, lib_dirs: &[LibData]) {
    generate_exec(project_root, base, lib_dirs, "test");
}

pub fn path_relative_to_dir(path: &path::Path, dir: &path::Path) -> String {
    let relative = dir.strip_prefix(path).expect("Could not strip prefix");
    relative.to_str().unwrap().into()
}

pub fn paths_to_relative<'a>(
    base: &path::Path,
    dirs: impl Iterator<Item = &'a path::PathBuf>,
) -> Vec<String> {
    dirs.map(|x| path_relative_to_dir(base, x.as_path()))
        .collect::<Vec<String>>()
}

pub fn generate_main(base: &path::Path, dirs: &[path::PathBuf]) {
    let mut data = BTreeMap::new();

    let dir_strings = paths_to_relative(base, dirs.iter());

    data.insert("DIRS", dir_strings.as_slice());

    let projectname = base.file_name().unwrap().to_str().unwrap();

    let replace = vec![("PROJECTNAME", projectname)];

    let outpath = base.join("CMakeLists.txt");
    let template_file = path::Path::new("templates/top.CMakeLists");

    write_template2(template_file, data, replace.as_slice(), outpath.as_path());
}
