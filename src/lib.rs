//! # polymesh_rw
//! polymesh_rw is a library for reading and writing meshes and simulation data in the OpenFOAM polyMesh file format.
//! Full case files can be read to a ```Case``` struct, which contains the mesh and all time directories.
//! ```
//! use polymesh_rw::*;
//! fn main() -> std::io::Result<()> {
//!            let case_file_path = std::path::Path::new("tests/test_cases/original/cylinder");
//!            let mut case = polymesh_rw::Case::parse_file(case_file_path)?;
//!            let boundary = &mut case.polymesh.boundary;
//!            
//!            let boundary_file_path = &case_file_path.join("constant/polyMesh/boundary");
//!            let boundary_2 = polymesh_rw::FileContent::<polymesh_rw::BoundaryData>::parse_file(&boundary_file_path)?;
//!            assert_eq!(*boundary, boundary_2);
//!            
//!            println!("{}", boundary);
//!            
//!            let polymesh_rw::FoamValue::Structure(ref mut down_bc) = boundary
//!                .data
//!                .get_mut("down")
//!                .expect("\"down\" boundary condition not found.")
//!            else {
//!                panic!("\"down\" boundary condition is not a structure.");
//!            };
//!            println!("{}", down_bc);
//!            *down_bc.get_mut("type").unwrap() = polymesh_rw::FoamValue::String("fixedValue".to_string());
//!            println!("{}", down_bc);
//!            
//!            let modified_case_file_path = std::path::Path::new("tests/test_cases/copy/cylinder");
//!            case.write_file(modified_case_file_path)?;
//!            // let modified_boundary_file_path =
//!            //     &modified_case_file_path.join("constant/polyMesh/boundary");
//!            // boundary.write_file(modified_boundary_file_path)?;
//!            Ok(())
//! }
//! ```
//! The data in a case struct is separated in a ```polymesh``` structure which stores the mesh, and a ```time_directories```
//! structure which stores the simulation data. For example, the boundary conditions which are located in the
//! ```constant/polyMesh/boundary``` file will be found in ```case.polymesh.boundary```.
//! Data files are stored in ```FileContent``` structs, which contain the metadata (header) and data of the file.
//! ```text
//! println!("{}", case.polymesh.boundary.data);
//! ```
//! All the data
//! and metadata containers implement ```std::fmt::Debug```, so they can be printed to the console.
//! ```text
//! println!("{}", case.polymesh.boundary.meta);
//! println!("{}", case.polymesh.boundary.data);
//! ```
use std::collections::HashMap;
use std::path;

mod base;
pub mod polymesh;

/// Re-export of structures that the user will want to interact with.

/// Containers of aggregated data.
pub use base::FileContent;
pub use polymesh::PolyMesh;
pub use polymesh::TimeDir;

/// Containers of individual files.
pub use polymesh::BoundaryData;
pub use polymesh::CellZone;
pub use polymesh::FaceData;
pub use polymesh::FaceZone;
pub use polymesh::NeighbourData;
pub use polymesh::OwnerData;
pub use polymesh::PointData;
pub use polymesh::PointZone;
pub use polymesh::ResultData;
pub use polymesh::Set;
pub use polymesh::Sets;
pub use polymesh::UniformData;
pub use polymesh::Zone;
pub use polymesh::ZoneData;

/// Containers of smaller pieces of data.
pub use base::FoamField;
pub use base::FoamFile;
pub use base::FoamStructure;
pub use base::FoamValue;

/// The Case structure holds the mesh and results found in a case directory.
#[derive(Debug, PartialEq, Clone)]
pub struct Case {
    pub polymesh: PolyMesh,
    pub time_directories: HashMap<String, TimeDir>,
}

impl Case {
    /// Parses the case directory and returns a Case struct.
    pub fn parse_file(dir_path: &path::Path) -> std::io::Result<Case> {
        let poly_mesh = PolyMesh::parse(&dir_path.join("constant/polyMesh"))?;
        let time_directories = numbered_directories(dir_path)?
            .iter()
            .map(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap().to_string();
                let time_directory = TimeDir::parse(entry).unwrap();
                (name, time_directory)
            })
            .collect();
        Ok(Case {
            polymesh: poly_mesh,
            time_directories,
        })
    }

    /// Writes the case contents to the given directory.
    pub fn write_file(&self, path: &path::Path) -> std::io::Result<()> {
        self.polymesh.write(path)?;
        for time_directory in self.time_directories.values() {
            time_directory.write(path)?;
        }
        Ok(())
    }
}

/// Returns a list of directories in the provided path whose names are numbers.
fn numbered_directories(path: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        // Check if the directory name is a number.
        let dir_name = path
            .file_name()
            .unwrap()
            .to_str()
            .expect("File name in case directory is not valid unicode.");
        if let Ok(_time) = dir_name.parse::<f64>() {
            dirs.push(path);
        }
    }
    Ok(dirs)
}

// #[cfg(test)]
// mod tests {
//     use crate::{BoundaryData, FileContent};
//     #[test]
//     fn test_consistency() -> std::io::Result<()> {
//         let case_file_path = std::path::Path::new("tests/test_cases/original/cylinder");
//         let mut case = crate::Case::parse_file(case_file_path)?;
//         let boundary = &mut case.polymesh.boundary;

//         let boundary_file_path = &case_file_path.join("constant/polyMesh/boundary");
//         let boundary_2 = FileContent::<BoundaryData>::parse_file(&boundary_file_path)?;
//         assert_eq!(*boundary, boundary_2);

//         println!("{}", boundary);

//         let crate::FoamValue::Structure(ref mut down_bc) = boundary
//             .data
//             .get_mut("down")
//             .expect("\"down\" boundary condition not found.")
//         else {
//             panic!("\"down\" boundary condition is not a structure.");
//         };
//         println!("{}", down_bc);
//         *down_bc.get_mut("type").unwrap() = crate::FoamValue::String("fixedValue".to_string());
//         println!("{}", down_bc);

//         let modified_case_file_path = std::path::Path::new("tests/test_cases/copy/cylinder");
//         boundary.write_file(modified_case_file_path)?;
//         let modified_boundary_file_path =
//             &modified_case_file_path.join("constant/polyMesh/boundary");
//         case.write_file(modified_boundary_file_path)?;
//         Ok(())
//     }
// }
