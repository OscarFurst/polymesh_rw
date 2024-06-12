use super::parser_base::*;
use super::writer_base::*;
use super::FileElement;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    number::complete::double,
    sequence::{delimited, preceded, terminated},
    IResult,
};

/// An enumerator that holds the different types of physical fields
/// that can be found in OpenFOAM files.
// TODO: nonuniform fields are declared as "List<_>" and could maybe be more generic.
#[derive(Debug, PartialEq, Clone)]
pub enum FoamField {
    UniformScalar(f64),
    UniformVector(Vec<f64>),
    Scalar(Vec<f64>),
    Vector(Vec<Vec<f64>>),
}

impl FileElement for FoamField {
    /// Parse a FoamField from the given input.
    fn parse(input: &str) -> IResult<&str, FoamField> {
        // starts with some information about the field
        let (input, field_type) = next(string_val)(input)?;
        match field_type.as_str() {
            "uniform" => parse_uniform(input),
            "nonuniform" => parse_nonuniform(input),
            _ => Err(nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Tag,
            })),
        }
    }
}

impl std::fmt::Display for FoamField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FoamField::UniformScalar(value) => writeln!(f, "uniform {};", value)?,
            FoamField::UniformVector(value) => {
                write!(f, "uniform (")?;
                write_vector_content(value, f)?;
                writeln!(f, ");")?
            }
            FoamField::Scalar(ref values) => {
                writeln!(f, "nonuniform List<scalar>")?;
                write_single_data(values, f)?;
                writeln!(f, ";")?
            }
            FoamField::Vector(ref values) => {
                writeln!(f, "nonuniform List<vector>")?;
                write_fixed_witdh_data(values, f)?;
                writeln!(f, ";")?
            }
        }
        Ok(())
    }
}

fn parse_uniform(input: &str) -> IResult<&str, FoamField> {
    terminated(
        alt((
            map(lws(double), FoamField::UniformScalar),
            map(
                lws(inline_parentheses(double_values)),
                FoamField::UniformVector,
            ),
        )),
        semicolon,
    )(input)
}

fn parse_nonuniform(input: &str) -> IResult<&str, FoamField> {
    // now comes "List<scalar>" or "List<vector>"
    terminated(
        preceded(
            delimited(lws(tag("List<")), string_val, char('>')),
            alt((scalar_field, vector_field)),
        ),
        semicolon,
    )(input)
}

fn scalar_field(input: &str) -> IResult<&str, FoamField> {
    map(double_scalar_field, FoamField::Scalar)(input)
}

fn vector_field(input: &str) -> IResult<&str, FoamField> {
    map(double_vector_field, FoamField::Vector)(input)
}
