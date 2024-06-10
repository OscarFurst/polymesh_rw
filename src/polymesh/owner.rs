use crate::base::single_i_data;
use crate::file_parser::FileParser;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct OwnerData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl FileParser for OwnerData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, OwnerData> {
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        Ok((input, OwnerData { n, cells }))
    }
}
