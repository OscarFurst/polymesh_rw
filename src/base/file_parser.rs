pub trait FileElement: Sized + std::fmt::Display + PartialEq {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

/// A trait for parsing and writing OpenFOAM files.
/// The trait takes care of the file header so that only the data parsing needs to be implemented.
pub trait FileParser: FileElement {
    /// Provides the relative position of the data file in the case directory.
    fn default_file_path() -> std::path::PathBuf;
}
