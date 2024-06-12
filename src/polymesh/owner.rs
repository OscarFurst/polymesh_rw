use crate::file_parser::FileParser;
use crate::parser_base::single_i_data;
use crate::writer_base::write_single_data;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct OwnerData {
    pub n: usize,
    pub cells: Vec<usize>,
}

impl FileParser for OwnerData {
    /// Assumes the remaining input contains the owner data.
    fn parse_data(input: &str) -> IResult<&str, OwnerData> {
        let (input, cells) = single_i_data(input)?;
        let n = cells.len();
        Ok((input, OwnerData { n, cells }))
    }

    fn default_file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/owner")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        write_single_data(&self.cells, file)
    }
}
