use crate::parser_base::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{opt, value},
    sequence::delimited,
    IResult,
};
use std::io::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FoamFileFormat {
    ascii,
    binary,
}

impl std::fmt::Display for FoamFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FoamFileFormat::ascii => write!(f, "ascii"),
            FoamFileFormat::binary => write!(f, "binary"),
        }
    }
}

fn parse_format(input: &str) -> IResult<&str, FoamFileFormat> {
    let parse_ascii = value(FoamFileFormat::ascii, tag("ascii"));
    let parse_binary = value(FoamFileFormat::binary, tag("binary"));
    alt((parse_ascii, parse_binary))(input)
}

fn parse_file_format(input: &str) -> IResult<&str, FoamFileFormat> {
    delimited(ws(tag("format")), ws(parse_format), semicolon)(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct FoamFileData {
    version: Option<String>,
    format: FoamFileFormat,
    class: String,
    note: Option<String>,
    location: String,
    object: String,
}

impl FoamFileData {
    /// Parse a FoamFile block if it exists.
    pub fn parse(input: &str) -> IResult<&str, FoamFileData> {
        // Consume the header
        let (input, _) = next(tag("FoamFile"))(input)?;
        let (input, _) = next(char('{'))(input)?;
        // Parse the contents
        let (input, version) = opt(next(key_string_semicolon("version")))(input)?;
        let (input, format) = next(parse_file_format)(input)?;
        let (input, class) = next(key_string_semicolon("class"))(input)?;
        let (input, note) = opt(next(key_string_semicolon("note")))(input)?;
        let (input, location) = next(key_string_semicolon("location"))(input)?;
        let (input, object) = next(key_string_semicolon("object"))(input)?;
        // Consume the footer
        let (input, _) = next(char('}'))(input)?;
        Ok((
            input,
            FoamFileData {
                version,
                format,
                class,
                note,
                location,
                object,
            },
        ))
    }

    pub fn parse_optional(input: &str) -> IResult<&str, Option<FoamFileData>> {
        opt(FoamFileData::parse)(input)
    }

    pub fn write_meta(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "FoamFile")?;
        writeln!(file, "{{")?;
        if let Some(version) = &self.version {
            writeln!(file, "    version     {};", version)?;
        }
        writeln!(file, "    format      {};", self.format)?;
        writeln!(file, "    class       {};", self.class)?;
        if let Some(note) = &self.note {
            writeln!(file, "    note        \"{}\";", note)?;
        }
        writeln!(file, "    location    \"{}\";", self.location)?;
        writeln!(file, "    object      {};", self.object)?;
        writeln!(file, "}}")?;
        writeln!(
            file,
            "// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * //\n"
        )
    }

    // Returns the relative path to the object from the case directory.
    pub fn relative_file_path(&self) -> String {
        self.location.clone() + "/" + &self.object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_foamfile() {
        let input = r#"
FoamFile
{
    format      ascii;
    class       vectorField;
    location    "constant/polyMesh";
    object      points;
}"#;
        let expected_data = FoamFileData {
            version: None,
            format: FoamFileFormat::ascii,
            class: "vectorField".to_string(),
            note: None,
            location: "constant/polyMesh".to_string(),
            object: "points".to_string(),
        };

        let expected = Ok(("", expected_data));
        let actual = FoamFileData::parse(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_stuff() {
        let input = r#"/*--------------------------------*- C++ -*----------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Version:  10
     \\/     M anipulation  |
\*---------------------------------------------------------------------------*/
FoamFile
{
    version     0.0;
    format      ascii;
    class       polyBoundaryMesh;
    note        "nPoints:215  nCells:592  nFaces:1388  nInternalFaces:980";
    location    "constant/polyMesh";
    object      boundary;
}"#;
        let expected_data = FoamFileData {
            version: Some("0.0".to_string()),
            format: FoamFileFormat::ascii,
            class: "polyBoundaryMesh".to_string(),
            note: Some(r#"nPoints:215  nCells:592  nFaces:1388  nInternalFaces:980"#.to_string()),
            location: r#"constant/polyMesh"#.to_string(),
            object: "boundary".to_string(),
        };

        let expected = Ok(("", expected_data));
        let actual = FoamFileData::parse(input);
        assert_eq!(expected, actual);
    }
}
