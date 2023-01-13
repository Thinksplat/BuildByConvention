use walkdir::{DirEntry, WalkDir};

// BuildDirType will be used to determine what kind of directory we are in
// with a paramter to the DirEntry
#[derive(Debug)]
pub enum BuildDirType {
    Lib,
    Test,
    App,
    Prototype,
}

pub struct BuildDirInfo {
    pub name: String,
    pub kind: BuildDirType,
    pub entry: DirEntry,
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

pub fn all_source_directories(root: &str) -> impl Iterator<Item = BuildDirInfo> {
    all_directories(root)
        .filter(ignore_dir)
        .filter_map(|e| entry_to_builddirtype(&e))
}

fn all_directories(root: &str) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_dir)
}

struct PrePostfix<'a> {
    prefix: &'a str,
    postfix: &'a str,
}

fn split_path_underscore(path_str: &str) -> Option<PrePostfix> {
    let undercase_sep = path_str.find('_')?;

    Some(PrePostfix {
        prefix: &path_str[..undercase_sep],
        postfix: &path_str[undercase_sep + 1..],
    })
}

#[test]
fn split_path_underscore_tests() {
    let split = split_path_underscore("lib_test");
    assert!(split.is_some());
    let split = split.unwrap();
    assert_eq!(split.prefix, "lib");
    assert_eq!(split.postfix, "test");
}

fn split_path_camelcase(path_str: &str) -> Option<PrePostfix> {
    let sep_index = path_str.find(|c: char| c.is_uppercase())?;

    Some(PrePostfix {
        prefix: &path_str[..sep_index],
        postfix: &path_str[sep_index..],
    })
}

#[test]
fn split_path_camelcase_tests() {
    let split = split_path_camelcase("libTest");
    assert!(split.is_some());
    let split = split.unwrap();
    assert_eq!(split.prefix, "lib");
    assert_eq!(split.postfix, "Test");
}

fn invalid_prepost(prepost: &PrePostfix) -> bool {
    prepost.prefix.is_empty() || prepost.postfix.is_empty()
}

fn valid_prepost(prepost: &PrePostfix) -> bool {
    !invalid_prepost(prepost)
}

/// Returns the prefix of a path string up to the first capital letter or underscore
fn split_path(path_str: &str) -> Option<PrePostfix> {
    let prepost = split_path_underscore(path_str).or_else(|| split_path_camelcase(path_str))?;
    valid_prepost(&prepost).then_some(prepost)
}

#[test]
fn split_path_tests() {
    let split = split_path("lib_test");
    assert!(split.is_some());
    let split = split.unwrap();
    assert_eq!(split.prefix, "lib");
    assert_eq!(split.postfix, "test");

    let split = split_path("libTest");
    assert!(split.is_some());
    let split = split.unwrap();
    assert_eq!(split.prefix, "lib");
    assert_eq!(split.postfix, "Test");
}

#[test]
fn split_path_tests_invalid() {
    let split = split_path("lib_");
    assert!(split.is_none());
}

fn prefix_to_dirtype(prefix: &str) -> Option<BuildDirType> {
    match prefix {
        "lib" => Option::Some(BuildDirType::Lib),
        "test" => Option::Some(BuildDirType::Test),
        "app" => Option::Some(BuildDirType::App),
        "prototype" => Option::Some(BuildDirType::Prototype),
        _ => Option::None,
    }
}

fn ignore_dir(entry: &DirEntry) -> bool {
    let path = entry.path();

    path.components().any(|c| c.as_os_str() == "build")
}

fn entry_to_builddirtype(entry: &DirEntry) -> Option<BuildDirInfo> {
    let path = entry.path();
    let filename_str = path.file_name()?.to_str()?;

    // prefix is the path_str up to _ or the first capital letter
    let prepost = split_path(filename_str)?;

    let dirtype = prefix_to_dirtype(prepost.prefix)?;

    Option::Some(BuildDirInfo {
        name: prepost.postfix.to_owned(),
        kind: dirtype,
        entry: entry.clone(),
    })
}
