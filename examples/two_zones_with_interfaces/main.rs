use polymesh_rw::polymesh::PolyMesh;

fn main() {
    // path from the root of the project because cargo run is executed from the root of the project
    let path = std::path::Path::new("./examples/two_zones_with_interfaces/constant/polyMesh/");
    let data = PolyMesh::parse(path).unwrap();
    println!("{:?}", data.facezones.data.facezones[0]);
}
