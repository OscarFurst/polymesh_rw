use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::IResult;

/// The NeighbourData structure holds the data of a polyMesh/neighbour file.
#[derive(Debug, PartialEq, Clone)]
pub struct NeighbourData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl std::ops::Deref for NeighbourData {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl std::ops::DerefMut for NeighbourData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl FileParser for NeighbourData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/neighbour")
    }
}

impl FileElement for NeighbourData {
    fn parse(input: &str) -> IResult<&str, NeighbourData> {
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        Ok((input, NeighbourData { n, cells }))
    }
}

impl std::fmt::Display for NeighbourData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_single_data(&self.cells, f)
    }
}
