use super::FileContent;
use crate::file_parser::FileParser;
use crate::parser_base::single_i_data;
use crate::writer_base::write_single_data;
use nom::IResult;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path;

#[derive(Debug, PartialEq, Clone)]
pub struct Sets {
    pub n: usize,
    pub sets: HashMap<String, FileContent<Set>>,
}

impl Sets {
    /// path is the path to the "sets" directory.
    pub fn parse(path: &path::Path) -> std::io::Result<Self> {
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
            let mut set =
                Set::parse(&path).expect(format!("Failed to parse set file {:?}.", path).as_str());
            // the name is provided afterwards because it is not stored in the data,
            // and its is required for the file_path method
            set.data.name = name.to_string();
            sets.insert(name.to_string(), set);
        }
        let n = sets.len();
        Ok(Self { n, sets })
    }

    pub fn write(&self, path: &path::Path) -> Result<(), Box<dyn Error>> {
        for set in self.sets.values() {
            set.write(path)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    pub name: String,
    pub n: usize,
    pub labels: Vec<usize>,
}

impl FileParser for Set {
    /// Assumes the remaining input contains the set data.
    fn parse_data(input: &str) -> IResult<&str, Set> {
        let (input, labels) = single_i_data(input)?;
        let n = labels.len();
        let name = "uninitialized".to_string();
        Ok((input, Set { name, n, labels }))
    }

    fn file_path(&self) -> path::PathBuf {
        path::PathBuf::from(format!("constant/polyMesh/sets/{}", self.name))
    }

    fn write_data(&self, file: &mut fs::File) -> std::io::Result<()> {
        write_single_data(&self.labels, file)
    }
}
