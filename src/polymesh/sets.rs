use super::FileContent;
use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::IResult;
use std::collections::HashMap;
use std::fs;
use std::path;

/// The Sets structure holds the full content of the "constant/polyMesh/sets" directory.
#[derive(Debug, PartialEq, Clone)]
pub struct Sets(pub HashMap<String, FileContent<Set>>);

impl std::ops::Deref for Sets {
    type Target = HashMap<String, FileContent<Set>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Sets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sets {
    /// Parses the "sets" directory and returns a Sets structure.
    /// path is the path to the "sets" directory.
    pub fn parse_files(path: &path::Path) -> std::io::Result<Self> {
        // check if the directory exists
        if !path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory {:?} not found.", path),
            ));
        }
        let set_files = fs::read_dir(path).unwrap();
        let mut sets = HashMap::new();
        for file in set_files {
            let file = file.unwrap();
            let path = file.path();
            let name = path.file_name().unwrap().to_str().unwrap();
            let mut set = FileContent::<Set>::parse_file(&path)
                .unwrap_or_else(|_| panic!("Failed to parse set file {:?}.", path));
            // the name is provided afterwards because it is not stored in the data
            set.data.name = name.to_string();
            sets.insert(name.to_string(), set);
        }
        Ok(Self(sets))
    }

    /// Writes the complete "sets" directory to the provided path.
    pub fn write(&self, path: &path::Path) -> std::io::Result<()> {
        for set in self.values() {
            set.write_file(path)?;
        }
        Ok(())
    }

    /// Returns the default path from the case directory to the "sets" directory.
    pub fn default_file_path(&self) -> path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/sets")
    }
}

/// The Set structure containts the data of a single set file found in the "constant/polyMesh/sets/" directory.
#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    pub name: String,
    pub n: usize,
    pub labels: Vec<usize>,
}

impl std::ops::Deref for Set {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.labels
    }
}

impl std::ops::DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.labels
    }
}

impl FileParser for Set {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/undefinedSetName")
    }
}

impl FileElement for Set {
    fn parse(input: &str) -> IResult<&str, Set> {
        let (input, labels) = single_i_data(input)?;
        let n = labels.len();
        let name = "uninitialized".to_string();
        Ok((input, Set { name, n, labels }))
    }
}

impl std::fmt::Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_single_data(&self.labels, f)
    }
}
