use super::{FileContent, ResultData, UniformData};
use std::collections::HashMap;

/// The structure that holds the full content of a time directory, which is where simulation results are stored.
#[derive(Debug, PartialEq, Clone)]
pub struct TimeDir {
    pub time: f64,
    // Keys: variable names.
    pub field_values: HashMap<String, FileContent<ResultData>>,
    pub uniform: Option<HashMap<String, FileContent<UniformData>>>,
}

impl TimeDir {
    pub fn parse(path: &std::path::Path) -> std::io::Result<TimeDir> {
        let Ok(time) = path.file_name().unwrap().to_str().unwrap().parse::<f64>() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Time directory name is not a valid number.",
            ));
        };
        let mut field_values = HashMap::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let name = path
                .file_name()
                .expect("Unable to extract file name while parsing time directory.")
                .to_str()
                .expect("File name in time directory is not valid unicode.")
                .to_string();
            let result_data = FileContent::<ResultData>::parse_file(&path)?;
            field_values.insert(name, result_data);
        }
        // check if there is a uniform directory
        let uniform = if path.join("uniform").is_dir() {
            let mut uniform = HashMap::new();
            for entry in std::fs::read_dir(path.join("uniform"))? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                let name = path
                    .file_name()
                    .expect("Unable to extract file name while parsing uniform directory.")
                    .to_str()
                    .expect("File name in uniform directory is not valid unicode.")
                    .to_string();
                let uniform_data = FileContent::<UniformData>::parse_file(&path)?;
                uniform.insert(name, uniform_data);
            }
            Some(uniform)
        } else {
            None
        };

        Ok(TimeDir {
            time,
            field_values,
            uniform,
        })
    }

    /// path: the path to the case directory.
    pub fn write(&self, path: &std::path::Path) -> std::io::Result<()> {
        for result in self.field_values.values() {
            result.write_file(path)?;
        }
        if let Some(uniform) = &self.uniform {
            for uniform in uniform.values() {
                uniform.write_file(path)?;
            }
        }
        Ok(())
    }
}
