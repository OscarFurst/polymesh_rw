use polymesh_rw::polymesh::PolyMesh;

fn main() {
    // multiple case files can be places in /examples/test_meshes/ and will be parsed to check if
    // the parser works correctly
    let base_path = std::path::Path::new("./examples/test_meshes/");
    let test_directories = std::fs::read_dir(base_path).expect("Failed to find test directories.");
    // parse all the test directories
    for test_directory in test_directories {
        let test_directory = test_directory.expect("Failed to read test directory.");
        println!("Parsing: {:?}", test_directory.path());
        let path = test_directory.path();
        let _ = PolyMesh::parse_and_assert(&path.join("constant/polyMesh/"));
    }
}
