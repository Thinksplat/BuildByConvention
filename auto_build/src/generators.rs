use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs;
use std::path;
use walkdir::DirEntry;

use crate::builddir::DirInfo;

fn generate_template(template_file: &path::Path, for_dir: &DirInfo) {
    let mut data = BTreeMap::new();
    data.insert("NAME", for_dir.name.to_owned());

    let path_to_write = for_dir.entry.path().join("CMakeLists.txt");

    write_template(template_file, data, path_to_write.as_path());
}

fn write_template(
    template_file: &path::Path,
    data: BTreeMap<&str, String>,
    path_to_write: &path::Path,
) {
    let template_contents =
        fs::read_to_string(template_file).expect("Could not read template file");
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

    fs::write(path_to_write, result).expect("Could not write CMakeLists.txt");
}

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

    fs::write(path_to_write, result).expect("Could not write CMakeLists.txt");
}

pub struct LibData {
    pub name : String
}

pub fn generate_lib(dir: &DirInfo) -> LibData {
    let template_file = path::Path::new("templates/lib.CMakeFiles");
    generate_template(template_file, dir);
    LibData { name: dir.name.to_string() }
}

pub fn generate_app(_dir: &DirEntry, _dirs: &[String]) {
    // let mut data = BTreeMap::new();

    // data.insert("LIBS", dirs);

    // let projectname = dir.file_name().unwrap().to_str().unwrap();

    // let replace = vec![("PROJECTNAME", projectname)];

    // let outpath = dir.join("CMakeLists.txt");
    // let template_file = path::Path::new("templates/top.CMakeFiles");

    // write_template2(template_file, data, replace.as_slice(), outpath.as_path());

}

pub fn generate_test(_dir: &DirEntry) {}

pub fn generate_prototype(_dir: &DirEntry) {}

pub fn generate_main(dir: &path::Path, dirs: &[String]) {
    let mut data = BTreeMap::new();

    data.insert("DIRS", dirs);

    let projectname = dir.file_name().unwrap().to_str().unwrap();

    let replace = vec![("PROJECTNAME", projectname)];

    let outpath = dir.join("CMakeLists.txt");
    let template_file = path::Path::new("templates/top.CMakeFiles");

    write_template2(template_file, data, replace.as_slice(), outpath.as_path());
}
