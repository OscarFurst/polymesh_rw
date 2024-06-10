use polymesh_rw::polymesh::PolyMesh;

fn main() {
    // parse writer_test_files/original
    let path = std::path::Path::new("./examples/writer_test_files/original/");
    println!("Parsing original: {:?}", path);
    let data = PolyMesh::parse_and_assert(&path.join("constant/polyMesh/"));
    // write the parsed data to writer_test_files/copy
    let path = std::path::Path::new("./examples/writer_test_files/copy/");
    println!("Writing copy: {:?}", path);
    data.write(&path).expect("Failed to write data.");
    // parse the copy
    println!("Parsing copy: {:?}", path);
    let copy = PolyMesh::parse_and_assert(&path.join("constant/polyMesh/"));
    // compare the original and the copy
    assert_eq!(data, copy);
}
