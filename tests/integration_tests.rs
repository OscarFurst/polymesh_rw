use polymesh_rw::Case;

#[test]
fn test_consistency() {
    let base_path = std::path::Path::new("./tests/test_meshes/");
    let original_path = base_path.join("original/");
    let test_directories =
        std::fs::read_dir(original_path).expect("Failed to find test directories.");
    for dir in test_directories {
        let path = dir.expect("Failed to read test directory.").path();
        // parse the original mesh
        println!("Parsing: {:?}", path);
        let data = Case::parse(&path).unwrap();
        let copy_path = base_path.join("copy/").join(path.file_name().unwrap());
        // write the parsed data to a different directory
        println!("Writing: {:?}", copy_path);
        data.write(&copy_path).expect("Failed to write data.");
        // parse the newly written, copied data
        let copy = Case::parse(&copy_path).unwrap();
        // compare the original and the copy
        assert_eq!(data.poly_mesh, copy.poly_mesh);
        assert_eq!(data.time_directories, copy.time_directories);
        assert_eq!(data, copy);
    }
}
