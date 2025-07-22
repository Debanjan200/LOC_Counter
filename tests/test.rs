use std::path::{Path, PathBuf};

use loc_counter::counter;

#[test]
fn test_for_c_file(){
    let path = Path::new("./test_sources/c.c");
    let comment_style_json_path = PathBuf::from("./comment_style_json_path/comment_style.json");

    let mp = counter::count_lines(path, None, &Vec::new(), Some(comment_style_json_path));

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 21);
    assert_eq!(stats.code, 8);
    assert_eq!(stats.comments, 8);
    assert_eq!(stats.blanks, 5);
}

#[test]
fn test_for_cpp_file(){
    let path = Path::new("./test_sources/cpp.cpp");
    let comment_style_json_path = PathBuf::from("./comment_style_json_path/comment_style.json");

    let mp = counter::count_lines(path, None, &Vec::new(), Some(comment_style_json_path));

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 21);
    assert_eq!(stats.code, 8);
    assert_eq!(stats.comments, 8);
    assert_eq!(stats.blanks, 5);
}

#[test]
fn test_for_rust_file(){
    let path = Path::new("./test_sources/rust.rs");
    let comment_style_json_path = PathBuf::from("./comment_style_json_path/comment_style.json");

    let mp = counter::count_lines(path, None, &Vec::new(), Some(comment_style_json_path));

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 22);
    assert_eq!(stats.code, 7);
    assert_eq!(stats.comments, 11);
    assert_eq!(stats.blanks, 4);
}

#[test]
fn test_for_java(){
    let path = Path::new("./test_sources/java.java");
    let comment_style_json_path = PathBuf::from("./comment_style_json_path/comment_style.json");

    let mp = counter::count_lines(path, None, &Vec::new(), Some(comment_style_json_path));

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 20);
    assert_eq!(stats.code, 8);
    assert_eq!(stats.comments, 8);
    assert_eq!(stats.blanks, 4);
}

#[test]
fn test_for_python(){
    let path = Path::new("./test_sources/python.py");

    let mp = counter::count_lines(path, None, &Vec::new(), None);

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 14);
    assert_eq!(stats.code, 3);
    assert_eq!(stats.comments, 7);
    assert_eq!(stats.blanks, 4);
}

#[test]
fn test_for_javascript(){
    let path = Path::new("./test_sources/javascript.js");

    let mp = counter::count_lines(path, None, &Vec::new(), None);

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 14);
    assert_eq!(stats.code, 4);
    assert_eq!(stats.comments, 6);
    assert_eq!(stats.blanks, 4);
}

#[test]
fn test_for_shell_script(){
    let path = Path::new("./test_sources/shell.sh");

    let mp = counter::count_lines(path, None, &Vec::new(), None);

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 8);
    assert_eq!(stats.code, 2);
    assert_eq!(stats.comments, 3);
    assert_eq!(stats.blanks, 3);
}

#[test]
fn test_for_shell(){
    let path = Path::new("./test_sources/config.yaml");

    let mp = counter::count_lines(path, None, &Vec::new(), None);

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 10);
    assert_eq!(stats.code, 4);
    assert_eq!(stats.comments, 3);
    assert_eq!(stats.blanks, 3);
}