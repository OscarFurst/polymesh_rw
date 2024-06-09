use crate::foam_file::FoamFileData;
use nom::IResult;

/// A trait for parsing OpenFOAM files.
/// The trait takes care of the file header so that only the data parsing needs to be implemented.
pub trait FileParser: Sized + PartialEq {
    fn parse(path: &str) -> Result<(Option<FoamFileData>, Self), &'static str> {
        // load file
        let input = std::fs::read_to_string(path).expect("Failed to read file.");
        // parse
        match Self::parse_all(&input) {
            Ok(("", data)) => Ok(data),
            _ => Err("Failed to parse data"),
        }
    }

    fn parse_all(input: &str) -> IResult<&str, (Option<FoamFileData>, Self)> {
        let (input, file_data) = FoamFileData::parse_optional(input)?;
        let (input, point_data) = Self::parse_data(input)?;
        Ok((input, (file_data, point_data)))
    }

    fn parse_data(input: &str) -> IResult<&str, Self>;
}
