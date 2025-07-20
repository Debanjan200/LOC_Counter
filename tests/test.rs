use std::path::Path;

use loc_counter::counter;

#[test]
fn test_for_c_file(){
    let path = Path::new("./test_sources/c.c");

    let mp = counter::count_lines(path, None, &Vec::new());

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

    let mp = counter::count_lines(path, None, &Vec::new());

    let path_buf = path.to_path_buf();

    assert!(mp.contains_key(&path_buf));
    let stats = mp.get(&path_buf).unwrap();

    assert_eq!(stats.total, 21);
    assert_eq!(stats.code, 8);
    assert_eq!(stats.comments, 8);
    assert_eq!(stats.blanks, 5);
}