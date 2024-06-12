use crate::base::{parser_base::*, FileElement, FoamStructure};
use crate::base::{FileParser, FoamValue};
use indexmap::map::IndexMap;
use nom::combinator::map;
use nom::{character::complete::char, multi::count, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct UniformData(FoamStructure);

impl std::ops::Deref for UniformData {
    type Target = FoamStructure;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FileParser for UniformData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("undefinedTimeDirectory/uniform/time")
    }
}

impl FileElement for UniformData {
    fn parse(input: &str) -> IResult<&str, UniformData> {
        map(FoamValue::parse_map, |content| {
            UniformData(FoamStructure {
                name: "".to_string(),
                content,
            })
        })(input)
    }
}

impl std::fmt::Display for UniformData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.display_content(f)
    }
}
