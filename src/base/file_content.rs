use super::parser_base::discard_garbage;
use super::FileElement;
use super::FileParser;
use super::FoamFileData;
use nom::IResult;

use std::io::prelude::*;

/// The FileContent structure holds the full content of a file.
/// The file content is divided into two parts:
/// - The meta data, which is written in the header and is a FoamStructure.
/// - The data, which is the actual content of the file. The data has a custom
///   structure depending on the file type.
/// The location is the path to the file relative to the case directory.
#[derive(Debug, PartialEq, Clone)]
pub struct FileContent<T: FileParser> {
    pub location: Option<std::path::PathBuf>,
    pub meta: FoamFileData,
    pub data: T,
}

impl<T: FileParser> FileElement for FileContent<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, meta) = FoamFileData::parse(input)?;
        let (input, data) = T::parse(input)?;
        let (input, _) = discard_garbage(input)?;
        Ok((
            input,
            FileContent {
                location: None,
                meta,
                data,
            },
        ))
    }
}

impl<T: FileParser> std::fmt::Display for FileContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.meta)?;
        writeln!(
            f,
            "// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * //"
        )?;
        writeln!(f, "{}", self.data)
    }
}

impl<T: FileParser> FileContent<T> {
    /// Create a new FileContent structure by parsing a file.
    pub fn parse_file(path: &std::path::Path) -> std::io::Result<Self> {
        // load file
        let input = std::fs::read_to_string(path)?;
        // find the path starting from the case directory
        let location = match find_case_directory(path) {
            Some(dir) => path.strip_prefix(dir).unwrap().to_path_buf().into(),
            None => None,
        };
        match Self::parse(&input) {
            Ok((rest, mut new_structure)) => {
                if !rest.is_empty() {
                    eprintln!(
                        "Warning: Parsing did not consume all input. Remaining: {}",
                        rest
                    );
                }
                new_structure.location = location;
                Ok(new_structure)
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("Failed to parse file {:?}: {}", path, e),
            )),
        }
    }

    /// Write the file to the given case directory.
    pub fn write_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let relative_path = match self.location {
            Some(ref p) => p.to_owned(),
            None => match self.meta.relative_file_path() {
                Some(p) => p,
                None => T::default_file_path(),
            },
        };
        let full_path = path.join(relative_path);
        if let Some(p) = full_path.parent() {
            std::fs::create_dir_all(p)?;
        }
        let mut file = std::fs::File::create(full_path)?;
        write!(&mut file, "{}", self)
    }
}

/// Checks if the given path is a directory that contains "constant" as a subdirectory.
fn is_case_directory(path: &std::path::Path) -> bool {
    path.is_dir() && path.join("constant").is_dir()
}

/// Searches for the case directory in the given path.
fn find_case_directory(path: &std::path::Path) -> Option<std::path::PathBuf> {
    if is_case_directory(path) {
        Some(path.to_path_buf())
    } else {
        path.ancestors()
            .find(|p| is_case_directory(p))
            .map(|p| p.to_path_buf())
    }
}
