use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct OwnerData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl FileParser for OwnerData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/owner")
    }
}

impl FileElement for OwnerData {
    /// Assumes the remaining input contains the owner data.
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
