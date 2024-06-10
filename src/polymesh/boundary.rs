use crate::file_parser::FileParser;
use crate::parser_base::*;
use nom::{character::complete::char, multi::count, IResult};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BoundaryData {
    pub n: usize,
    pub boundaries: HashMap<String, Boundary>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boundary {
    pub name: String,
    pub boundary_type: String,
    pub physical_type: String,
    pub n_faces: usize,
    pub start_face: usize,
}

fn parse_boundary(input: &str) -> IResult<&str, Boundary> {
    // starts with name
    let (input, name) = next(string_val)(input)?;
    // opening curly brace
    let (input, _) = next(char('{'))(input)?;
    // type
    let (input, boundary_type) = next(key_string_semicolon("type"))(input)?;
    // physicalType
    let (input, physical_type) = next(key_string_semicolon("physicalType"))(input)?;
    // nFaces
    let (input, n_faces) = next(key_usize_semicolon("nFaces"))(input)?;
    // startFace
    let (input, start_face) = next(key_usize_semicolon("startFace"))(input)?;
    // closing curly brace
    let (input, _) = next(char('}'))(input)?;
    Ok((
        input,
        Boundary {
            name,
            boundary_type,
            physical_type,
            n_faces,
            start_face,
        },
    ))
}

impl FileParser for BoundaryData {
    /// Assumes the remaining input contains the data.
    fn parse_data(input: &str) -> IResult<&str, BoundaryData> {
        // number of boundaries
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse boundaries
        let (input, boundary_vector) = count(parse_boundary, n)(input)?;
        let mut boundaries = HashMap::new();
        for boundary in &boundary_vector {
            boundaries.insert(boundary.name.clone(), boundary.clone());
        }
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, BoundaryData { n, boundaries }))
    }
}
