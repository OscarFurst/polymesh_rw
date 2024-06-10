use crate::base::*;
use crate::file_parser::FileParser;
use nom::{character::complete::char, multi::count, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct PointZoneData {
    pub n: usize,
    pub pointzones: Vec<PointZone>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointZone {
    // starts with a "type" which I have only seen as "pointZone", so I'm not storing it for now
    pub name: String,
    pub n: usize,
    pub points: Vec<usize>,
}

fn parse_point_zone(input: &str) -> IResult<&str, PointZone> {
    // starts with name
    let (input, name) = next(string_val)(input)?;
    let name = name.to_string();
    // opening curly brace
    let (input, _) = next(char('{'))(input)?;
    // "    type pointZone;"
    let (input, _) = known_key_value_semicolon("type", "pointZone")(input)?;
    // "pointLabels      List<label> "
    let (input, _) = known_key_value("pointLabels", "List<label>")(input)?;
    // list of points
    let (input, points) = single_i_data(input)?;
    let n = points.len();
    // closing semicolon
    let (input, _) = next(semicolon)(input)?;
    // closing curly brace
    let (input, _) = next(char('}'))(input)?;
    Ok((
        input,
        PointZone {
            name,
            n,
            points: points,
        },
    ))
}

impl FileParser for PointZoneData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, PointZoneData> {
        // number of point zones
        let (input, n) = next(usize_val)(input)?;
        // opening parenthesis
        let (input, _) = next(char('('))(input)?;
        // parse point zones
        let (input, pointzones) = count(parse_point_zone, n)(input)?;
        // closing parenthesis
        let (input, _) = next(char(')'))(input)?;
        Ok((input, PointZoneData { n, pointzones }))
    }
}
