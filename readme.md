# polymesh_rw

A basic rust crate for reading and writing meshes and simulation data in the polymesh format used by OpenFOAM.

## Features

**This crate is a work in progress.** The parser was only tested on a small set of meshes. **If your mesh is not parsed correctly, please contribute by opening an issue and uploading your mesh.**

Curretly, following features are implemented:
- [x] Read mesh
- [x] Write mesh
- [x] Read results
- [x] Write results
- [ ] Practical constructors for each file type.
- [ ] Parse more common OpenFoam data types (unrecognized types are currently parsed as Strings.)
- [ ] Binary file formats
- [ ] Data consistency checks

## Contribute

Please feel encouraged to contribute advice, test cases and code.