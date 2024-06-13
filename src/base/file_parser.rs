/// A trait for parsing and writing pieces of OpenFOAM files.
pub trait FileElement: Sized + std::fmt::Display + PartialEq {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

/// A trait for parsing and writing OpenFOAM files.
/// Structures that implement this trait containt the full data of a file, which
/// is generally divided into multiple FileElements.
pub trait FileParser: FileElement {
    /// Provides the relative position of the data file in the case directory.
    fn default_file_path() -> std::path::PathBuf;
}
