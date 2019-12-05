use std::env;
use std::path;
extern crate id_pak;

/// Test opening and reading from a PAK
/// which contains:
///
/// * 1x1 pixel red.png.
/// * 1x1 pixel blue.gif.
/// * 1x1 pixel green.jpg.
#[test]
fn test_open_and_read_pak() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pak_path: path::PathBuf = [manifest_dir.as_str(), "tests", "test.pak"]
        .iter()
        .collect();

    let pak = id_pak::open(path::Path::new(&pak_path)).unwrap();
    assert_eq!(pak.get_file_count(), 3);
}
