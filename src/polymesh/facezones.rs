use crate::base::*;
use crate::file_parser::FileParser;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct FaceZoneData {
    pub n: usize,
    pub facezones: Vec<FaceZone>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FaceZone {
    // starts with a "type" which I have only seen as "faceZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub faces: Vec<usize>,
    // no idea what flipmap is does and what forms it can take, only seen one example so far
    // maybe even useless : https://gitlab.kitware.com/vtk/vtk/-/issues/17103
    pub flipmap: bool,
}

fn parse_flipmap(input: &str, n: usize) -> IResult<&str, bool> {
    delimited(
        tag(n.to_string().as_str()),
        delimited(char('{'), bool, char('}')),
        semicolon,
    )(input)
}

fn parse_face_zone(input: &str) -> IResult<&str, FaceZone> {
    // starts with name
    let (input, name) = next(string_val)(input)?;
    // opening curly brace
    let (input, _) = next(char('{'))(input)?;
    // "    type faceZone;"
    let (input, _) = next(known_key_value_semicolon("type", "faceZone"))(input)?;
    // "faceLabels      List<label> "
    let (input, _) = next(known_key_value("faceLabels", "List<label>"))(input)?;
    // list of faces
    let (input, faces) = single_i_data(input)?;
    let n = faces.len();
    // closing semicolon
    let (input, _) = next(semicolon)(input)?;
    // "flipMap         List<bool>"
    let (input, _) = next(known_key_value("flipMap", "List<bool>"))(input)?;
    let (input, _) = discard_empty(input)?;
    // on the same line: <number of faces>{<bool>};
    let (input, flipmap) = parse_flipmap(input, n)?;
    // closing curly brace
    let (input, _) = next(char('}'))(input)?;
    Ok((
        input,
        FaceZone {
            name,
            n,
            faces,
            flipmap,
        },
    ))
}

impl FileParser for FaceZoneData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, FaceZoneData> {
        // number of face zones
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse face zones
        let (input, facezones) = count(parse_face_zone, n)(input)?;
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, FaceZoneData { n, facezones }))
    }
}
