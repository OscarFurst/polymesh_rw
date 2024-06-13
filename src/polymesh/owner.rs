use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::IResult;

/// The OwnerData structure holds the data of a polyMesh/owner file.
#[derive(Debug, PartialEq, Clone)]
pub struct OwnerData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl std::ops::Deref for OwnerData {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl std::ops::DerefMut for OwnerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl FileParser for OwnerData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/owner")
    }
}

impl FileElement for OwnerData {
    fn parse(input: &str) -> IResult<&str, OwnerData> {
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        Ok((input, OwnerData { n, cells }))
    }
}

impl std::fmt::Display for OwnerData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_single_data(&self.cells, f)
    }
}
