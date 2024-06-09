use crate::base::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, value},
    sequence::delimited,
    IResult,
};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FoamFileFormat {
    ascii,
    binary,
}

fn parse_format(input: &str) -> IResult<&str, FoamFileFormat> {
    let parse_ascii = value(FoamFileFormat::ascii, tag("ascii"));
    let parse_binary = value(FoamFileFormat::binary, tag("binary"));
    alt((parse_ascii, parse_binary))(input)
}

fn parse_file_format(input: &str) -> IResult<&str, FoamFileFormat> {
    delimited(ws(tag("format")), ws(parse_format), semicolon)(input)
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FoamFileClass {
    scalarField,
    vectorField,
}

fn parse_class(input: &str) -> IResult<&str, FoamFileClass> {
    let parse_scalar_field = value(FoamFileClass::scalarField, tag("scalarField"));
    let parse_vector_field = value(FoamFileClass::vectorField, tag("vectorField"));
    alt((parse_scalar_field, parse_vector_field))(input)
}

fn parse_file_class(input: &str) -> IResult<&str, FoamFileClass> {
    delimited(ws(tag("class")), ws(parse_class), semicolon)(input)
}

fn parse_location(input: &str) -> IResult<&str, &str> {
    delimited(ws(tag("location")), ws(key), semicolon)(input)
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FoamFileObject {
    points,
    faces,
}

fn parse_object(input: &str) -> IResult<&str, FoamFileObject> {
    let parse_points = value(FoamFileObject::points, tag("points"));
    let parse_faces = value(FoamFileObject::faces, tag("faces"));
    alt((parse_points, parse_faces))(input)
}

fn parse_file_object(input: &str) -> IResult<&str, FoamFileObject> {
    delimited(ws(tag("object")), ws(parse_object), semicolon)(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct FoamFileData {
    format: FoamFileFormat,
    class: FoamFileClass,
    location: String,
    object: FoamFileObject,
}

impl FoamFileData {
    /// Parse a FoamFile block if it exists.
    pub fn parse(input: &str) -> IResult<&str, FoamFileData> {
        // Consume the header
        let (input, _) = next(tag("FoamFile"))(input)?;
        let (input, _) = next(tag("{"))(input)?;
        // Parse the contents
        let (input, format) = next(parse_file_format)(input)?;
        let (input, class) = next(parse_file_class)(input)?;
        let (input, location) = next(parse_location)(input)?;
        let (input, object) = next(parse_file_object)(input)?;
        // Consume the footer
        let (input, _) = next(tag("}"))(input)?;
        Ok((
            input,
            FoamFileData {
                format,
                class,
                location: location.to_string(),
                object,
            },
        ))
    }

    pub fn parse_optional(input: &str) -> IResult<&str, Option<FoamFileData>> {
        let (input, file_data) = opt(FoamFileData::parse)(input)?;
        Ok((input, file_data))
    }
}
mod tests {
    use crate::foam_file::*;

    #[test]
    fn test_parse_foamfile() {
        let input = r#"FoamFile
{
    format      ascii;
    class       vectorField;
    location    "constant/polyMesh";
    object      points;
}"#;
        let expected_data = FoamFileData {
            format: FoamFileFormat::ascii,
            class: FoamFileClass::vectorField,
            location: "constant/polyMesh".to_string(),
            object: FoamFileObject::points,
        };

        let expected = Ok(("", expected_data));
        let actual = FoamFileData::parse(input);
        assert_eq!(expected, actual);
    }
}
