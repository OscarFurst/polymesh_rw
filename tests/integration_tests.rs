use polymesh_rw::polymesh::PolyMesh;

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
        let data = PolyMesh::parse_and_assert(&path.join("constant/polyMesh/"));
        let copy_path = base_path.join("copy/").join(path.file_name().unwrap());
        // write the parsed data to a different directory
        println!("Writing: {:?}", copy_path);
        data.write(&copy_path).expect("Failed to write data.");
        // parse the newly written, copied data
        let copy = PolyMesh::parse_and_assert(&copy_path.join("constant/polyMesh/"));
        // compare the original and the copy
        assert_eq!(data, copy);
    }
}
