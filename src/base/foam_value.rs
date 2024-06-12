use super::foam_field::FoamField;
use super::foam_structure::FoamStructure;
use super::parser_base::*;
use super::writer_base::*;
use super::FileElement;
use nom::bytes::complete::tag;
use nom::multi::count;
use nom::sequence::preceded;
use nom::{
    bytes::complete::take_till,
    combinator::{map, map_res},
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum FoamValue {
    String(String),
    Integer(usize),
    Float(f64),
    Field(FoamField),
    // TODO: Lists should be more generic.
    List(Vec<String>),
    Structure(FoamStructure),
}

impl FileElement for FoamValue {
    /// Parse a FoamValue from the given input. A FoamValue can span multiple lines and
    /// contain a FoamStructure itself.
    fn parse(input: &str) -> IResult<&str, FoamValue> {
        // Check if if its a positive integer.
        if let Ok((input, value)) = map_res(
            terminated(ws(take_till(|c| c == ';')), semicolon),
            str::parse::<usize>,
        )(input)
        {
            return Ok((input, FoamValue::Integer(value)));
        }
        // Check if its a float.
        // TODO: Negative integers will be parsed as floats. Is this a problem?
        // TODO: Some things that can be parsed as floats might need to be parsed as Strings, e.g. versions!
        if let Ok((input, value)) = map_res(
            terminated(ws(take_till(|c| c == ';')), semicolon),
            str::parse::<f64>,
        )(input)
        {
            return Ok((input, FoamValue::Float(value)));
        }
        // Check if it is a field.
        if let Ok((input, value)) = terminated(FoamField::parse, semicolon)(input) {
            return Ok((input, FoamValue::Field(value)));
        }
        // Check if it is a list.
        if let Ok((input, value)) = terminated(parse_list, semicolon)(input) {
            return Ok((input, FoamValue::List(value)));
        }
        // Check if it is a structure.
        if let Ok((input, value)) = FoamStructure::parse_content(input) {
            return Ok((input, FoamValue::Structure(value)));
        }
        // If none of the above, it is a string or something that is not implemented yet.
        map(terminated(take_till(|c| c == ';'), semicolon), |s| {
            FoamValue::String(s.to_string())
        })(input)
    }
}

impl std::fmt::Display for FoamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FoamValue::String(value) => writeln!(f, "{};", value)?,
            FoamValue::Integer(value) => writeln!(f, "{};", value)?,
            FoamValue::Float(value) => writeln!(f, "{};", value)?,
            FoamValue::Field(value) => {
                writeln!(f, "{};", value)?;
            }
            FoamValue::List(values) => {
                write!(f, "List<word> ")?;
                write_single_data(values, f)?;
                writeln!(f, ";")?
            }
            FoamValue::Structure(value) => value.write_recursive(f)?,
        }
        Ok(())
    }
}

// TODO: Lists are found in a lot of places, so they should be
// more generic and used in FoamField for examples.
/// Parses a List<word>.
fn parse_list(input: &str) -> IResult<&str, Vec<String>> {
    preceded(tag("List<word>"), parse_list_content)(input)
}

fn parse_list_content(input: &str) -> IResult<&str, Vec<String>> {
    let (input, n) = lws(usize_val)(input)?;
    block_parentheses(count(lws(string_val), n))(input)
}
