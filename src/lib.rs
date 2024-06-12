use std::collections::HashMap;
use std::path;

pub mod base;
pub mod polymesh;

pub use polymesh::{PolyMesh, TimeDir};

#[derive(Debug, PartialEq, Clone)]
pub struct Case {
    pub poly_mesh: PolyMesh,
    pub time_directories: HashMap<String, TimeDir>,
}

impl Case {
    /// Parses the case directory and returns a Case struct.
    pub fn parse(dir_path: &path::Path) -> std::io::Result<Case> {
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
            poly_mesh,
            time_directories,
        })
    }

    /// Writes the case contents to the given directory.
    pub fn write(&self, path: &path::Path) -> std::io::Result<()> {
        self.poly_mesh.write(path)?;
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
