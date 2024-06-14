[![Crates.io Version](https://img.shields.io/crates/v/polymesh_rw)](https://crates.io/crates/polymesh_rw)

# polymesh_rw

polymesh_rw is a library for reading and writing meshes and simulation data in the OpenFOAM polyMesh file format.

## Features

**This crate is a work in progress.** The parser was only tested on a small set of meshes. **If your mesh is not parsed correctly, please contribute by opening an issue and uploading your mesh.**

Curretly, following features are implemented:
- [x] Read mesh
- [x] Write mesh
- [x] Read results
- [x] Write results
- [ ] Parse more common OpenFoam data types (unrecognized types are currently parsed as Strings.)
- [ ] Binary file formats
- [ ] Data consistency checks

## Example

Full case files can be read to a ```Case``` structure, which contains the mesh and all time directories.
```rust
use polymesh_rw::*;
let case_file_path = std::path::Path::new("tests/test_cases/original/cylinder");
let mut case = Case::parse_file(case_file_path)?;
```
The data in a case struct is separated in a ```polymesh``` structure which stores the mesh, and a ```time_directories``` structure which stores the simulation data. For example, the boundary conditions which are located in the ```constant/polyMesh/boundary``` file will be found in ```case.polymesh.boundary```.
```rust
let boundary = &mut case.polymesh.boundary;
```
Data files are stored in ```FileContent``` structures, which contain the metadata (header) and data of the file. The structure also allows to parse and write files individually.
```rust
let boundary_file_path = &case_file_path.join("constant/polyMesh/boundary");
let boundary_2 = FileContent::<BoundaryData>::parse_file(&boundary_file_path)?;
assert_eq!(*boundary, boundary_2);
```
All the data and metadata containers implement ```std::fmt::Debug```, so they can be printed to the console.
```rust
println!("{}", boundary);
```
The underlying data is stored in two different ways: either as HashMaps or a Vectors. The wrappers around these data types, which provide parsing and writing functionality, also implement Deref and DerefMut for easy manipulation.
Inside of the FoamStructures (HashMaps) the data is stored as FoamValues, which indicate the type of the data:
- String
- Float
- Integer
- List
- Structure
```rust
let FoamValue::Structure(ref mut down_bc) = boundary
    .data
    .get_mut("down")
    .expect("\"down\" boundary condition not found.")
else {
    panic!("\"down\" boundary condition is not a structure.");
};
println!("{}", down_bc);
*down_bc.get_mut("type").unwrap() = FoamValue::String("fixedValue".to_string());
println!("{}", down_bc);
```
Files can be written using the ```write_file``` method, which writes the data to the provided path.
In the following example, the full case is written to a new directory.
```rust
let modified_case_file_path = std::path::Path::new("tests/test_cases/copy/cylinder");
case.write_file(modified_case_file_path)?;
```
We can also choose to write only the ```boundary``` file, which is a part of the full case.
```rust
let modified_case_file_path = std::path::Path::new("tests/test_cases/copy/cylinder");
boundary.write_file(modified_case_file_path)?;
```
We still provide the path to the case directory, but the file will be written to the correct location inside the case directory. If the relative location needs to be changed, it can be done by assigning the correct *relative* path to the ```boundary.meta.location``` field (relative to the case directory).

## Contribute

Please feel encouraged to contribute advice, test cases and code.