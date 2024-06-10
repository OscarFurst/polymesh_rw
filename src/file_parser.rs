use crate::foam_file::FoamFileData;
use crate::parser_base::discard_garbage;
use crate::polymesh::FileContent;
use nom::IResult;
use std::path;

/// A trait for parsing OpenFOAM files.
/// The trait takes care of the file header so that only the data parsing needs to be implemented.
pub trait FileParser: Sized + PartialEq {
    /// Parse the file at the given path.
    fn parse(path: &path::Path) -> Result<FileContent<Self>, String> {
        // load file
        let input = std::fs::read_to_string(path)
            .expect(format!("Failed to read file {:?}.", path).as_str());
        // parse
        match parse_all(&input) {
            Ok((_, (meta, data))) => Ok(FileContent { meta, data }),
            Err(e) => Err(format!("Failed to parse file {:?}: {}", path, e)),
        }
    }

    /// Parse the file at the given path and check if the file was read until the end.
    fn parse_and_check(path: &path::Path) -> Result<FileContent<Self>, String> {
        // load file
        let input = std::fs::read_to_string(path).expect("Failed to read file.");
        // parse
        match parse_all(&input) {
            Ok(("", (meta, data))) => Ok(FileContent { meta, data }),
            Ok((rest, (meta, data))) => {
                eprintln!(
                    "Warning: Parsing did not consume all input. Remaining: {}",
                    rest
                );
                Ok(FileContent { meta, data })
            }
            Err(e) => Err(format!("Failed to parse file {:?}: {}", path, e)),
        }
    }

    /// Parse the file at the given path and check if the file was read until the end.
    fn parse_and_assert(path: &path::Path) -> FileContent<Self> {
        // load file
        let input = std::fs::read_to_string(path).expect("Failed to read file.");
        // parse
        match parse_all(&input) {
            Ok(("", (meta, data))) => FileContent { meta, data },
            Ok((rest, _)) => {
                panic!(
                    "Error: Parsing did not consume all input. Remaining: {}",
                    rest
                );
            }
            Err(e) => panic!("Failed to parse file {:?}: {}", path, e),
        }
    }

    /// Parse the data part of the file.
    fn parse_data(input: &str) -> IResult<&str, Self>;

    /// Write the data part of the file.
    /// path: The path to the the case directory.
    fn write_data(&self, path: &path::Path) -> Result<(), String>;

    /// Provides the relative position of the data file in the case directory.
    fn file_path() -> path::PathBuf;
}

/// Combine the parsing of the file header and the data.
fn parse_all<T: FileParser>(input: &str) -> IResult<&str, (Option<FoamFileData>, T)> {
    let (input, file_data) = FoamFileData::parse_optional(input)?;
    let (input, point_data) = T::parse_data(input)?;
    let (input, _) = discard_garbage(input)?;
    Ok((input, (file_data, point_data)))
}
