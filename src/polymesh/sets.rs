use super::FileContent;
use crate::file_parser::FileParser;
use crate::parser_base::single_i_data;
use nom::IResult;
use std::collections::HashMap;
use std::fs;
use std::path;

pub struct Sets {
    pub n: usize,
    pub sets: HashMap<String, FileContent<Set>>,
}

impl Sets {
    /// path is the path to the "sets" directory.
    pub fn parse(path: &path::Path) -> Self {
        let set_files = fs::read_dir(path).unwrap();
        let mut sets = HashMap::new();
        for file in set_files {
            let file = file.unwrap();
            let path = file.path();
            let name = path.file_name().unwrap().to_str().unwrap();
            let set =
                Set::parse(&path).expect(format!("Failed to parse set file {:?}.", path).as_str());
            sets.insert(name.to_string(), set);
        }
        let n = sets.len();
        Self { n, sets }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    pub n: usize,
    pub labels: Vec<usize>,
}

impl FileParser for Set {
    /// Assumes the remaining input contains the data.
    fn parse_data(input: &str) -> IResult<&str, Set> {
        let (input, labels) = single_i_data(input)?;
        let n = labels.len();
        Ok((input, Set { n, labels }))
    }
}
