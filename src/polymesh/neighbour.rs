use crate::file_parser::FileParser;
use crate::parser_base::single_i_data;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct NeighbourData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl FileParser for NeighbourData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, NeighbourData> {
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        Ok((input, NeighbourData { n, cells }))
    }
}
