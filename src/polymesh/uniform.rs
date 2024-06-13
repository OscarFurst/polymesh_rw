use crate::base::FileParser;
use crate::base::{FileElement, FoamStructure};
use nom::combinator::map;
use nom::IResult;

/// The UniformData structure holds the data of a polyMesh/uniform file.
#[derive(Debug, PartialEq, Clone)]
pub struct UniformData(pub FoamStructure);

impl std::ops::Deref for UniformData {
    type Target = FoamStructure;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for UniformData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FileParser for UniformData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("undefinedTimeDirectory/uniform/time")
    }
}

impl FileElement for UniformData {
    fn parse(input: &str) -> IResult<&str, UniformData> {
        map(FoamStructure::parse, UniformData)(input)
    }
}

impl std::fmt::Display for UniformData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // calling "write!(f, "{}", self)" worked in the past (because it was dereferenced correctly),
        // but after a completely unrelated modification (implementation of "write_foam_file")
        // it causes a stack overflow???
        write!(f, "{}", self.0)
    }
}
