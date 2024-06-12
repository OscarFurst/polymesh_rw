use crate::foam_file::FoamFileData;
use crate::parser_base::discard_garbage;
use crate::polymesh::FileContent;
use nom::IResult;
use std::path;

/// A trait for parsing OpenFOAM files.
/// The trait takes care of the file header so that only the data parsing needs to be implemented.
pub trait FileParser: Sized + PartialEq {
    /// Parse the file at the given path.
    fn parse(path: &path::Path) -> std::io::Result<FileContent<Self>> {
        // load file
        let input = std::fs::read_to_string(path)?;
        // find the path starting from the case directory
        let location = match find_case_directory(path) {
            Some(dir) => path.strip_prefix(dir).unwrap().to_path_buf().into(),
            None => None,
        };
        // parse
        match parse_all(&input) {
            Ok((_, (meta, data))) => Ok(FileContent {
                location,
                meta,
                data,
            }),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("Failed to parse file {:?}: {}", path, e),
            )),
        }
    }

    /// Parse the file at the given path and check if the file was read until the end.
    fn parse_and_check(path: &path::Path) -> std::io::Result<FileContent<Self>> {
        // load file
        let input = std::fs::read_to_string(path)?;
        // find the path starting from the case directory
        let location = match find_case_directory(path) {
            Some(dir) => path.strip_prefix(dir).unwrap().to_path_buf().into(),
            None => None,
        };
        // parse
        match parse_all(&input) {
            Ok((rest, (meta, data))) => {
                if !rest.is_empty() {
                    eprintln!(
                        "Warning: Parsing did not consume all input. Remaining: {}",
                        rest
                    );
                }
                Ok(FileContent {
                    location,
                    meta,
                    data,
                })
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("Failed to parse file {:?}: {}", path, e),
            )),
        }
    }

    /// Parse the file at the given path and check if the file was read until the end.
    fn parse_and_assert(path: &path::Path) -> std::io::Result<FileContent<Self>> {
        // load file
        let input = std::fs::read_to_string(path)?;
        // find the path starting from the case directory
        let location = match find_case_directory(path) {
            Some(dir) => path.strip_prefix(dir).unwrap().to_path_buf().into(),
            None => None,
        };
        // parse
        match parse_all(&input) {
            Ok((rest, (meta, data))) => {
                if !rest.is_empty() {
                    panic!(
                        "Warning: Parsing did not consume all input. Remaining: {}",
                        rest
                    );
                }
                Ok(FileContent {
                    location,
                    meta,
                    data,
                })
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("Failed to parse file {:?}: {}", path, e),
            )),
        }
    }

    /// Parse the data part of the file.
    fn parse_data(input: &str) -> IResult<&str, Self>;

    /// Write the data part of the provided file.
    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()>;

    /// Provides the relative position of the data file in the case directory.
    fn default_file_path(&self) -> path::PathBuf;
}

/// Combine the parsing of the file header and the data.
fn parse_all<T: FileParser>(input: &str) -> IResult<&str, (FoamFileData, T)> {
    let (input, file_data) = FoamFileData::parse(input)?;
    let (input, point_data) = T::parse_data(input)?;
    let (input, _) = discard_garbage(input)?;
    Ok((input, (file_data, point_data)))
}

/// Checks if the given path is a directory that contains "constant" as a subdirectory.
fn is_case_directory(path: &path::Path) -> bool {
    path.is_dir() && path.join("constant").is_dir()
}

/// Searches for the case directory in the given path.
fn find_case_directory(path: &path::Path) -> Option<path::PathBuf> {
    if is_case_directory(path) {
        Some(path.to_path_buf())
    } else {
        path.ancestors()
            .find(|p| is_case_directory(p))
            .map(|p| p.to_path_buf())
    }
}
