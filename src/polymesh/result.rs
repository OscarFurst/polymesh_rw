use crate::file_parser::FileParser;
use crate::parser_base::*;
use crate::writer_base::*;
use nom::combinator::map;
use nom::number::complete::double;
use nom::sequence::terminated;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};
use std::io::prelude::*;

// TODO : this is missing "boundaryField" information !!

#[derive(Debug, PartialEq, Clone)]
enum ResultType {
    UniformScalar(f64),
    UniformVector(Vec<f64>),
    Scalar(Vec<f64>),
    Vector(Vec<Vec<f64>>),
}

fn parse_result(input: &str) -> IResult<&str, ResultType> {
    // starts with some information about the field
    let (input, field_type) = next(key_string("internalField"))(input)?;
    match field_type.as_str() {
        "uniform" => parse_uniform(input),
        "nonuniform" => parse_nonuniform(input),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

fn parse_uniform(input: &str) -> IResult<&str, ResultType> {
    terminated(
        alt((
            map(lws(double), |v| ResultType::UniformScalar(v)),
            map(lws(inline_parentheses(double_values)), |v| {
                ResultType::UniformVector(v)
            }),
        )),
        char(';'),
    )(input)
}

fn parse_nonuniform(input: &str) -> IResult<&str, ResultType> {
    // now comes "List<scalar>" or "List<vector>"
    preceded(
        delimited(lws(tag("List<")), string_val, char('>')),
        alt((scalar_field, vector_field)),
    )(input)
}

fn scalar_field(input: &str) -> IResult<&str, ResultType> {
    let (input, values) = double_scalar_field(input)?;
    Ok((input, ResultType::Scalar(values)))
}

fn vector_field(input: &str) -> IResult<&str, ResultType> {
    let (input, values) = double_vector_field(input)?;
    Ok((input, ResultType::Vector(values)))
}

type Dimensions = [i32; 7];

fn dimensions_string(dimensions: &Dimensions) -> String {
    format!(
        "dimensions      [{} {} {} {} {} {} {}];",
        dimensions[0],
        dimensions[1],
        dimensions[2],
        dimensions[3],
        dimensions[4],
        dimensions[5],
        dimensions[6]
    )
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(preceded(opt(char('-')), digit1)),
        str::parse::<i32>,
    )(input)
}

fn dimension(input: &str) -> IResult<&str, Dimensions> {
    map_res(
        delimited(char('['), count(lws(parse_i32), 7), tag("];")),
        Vec::try_into,
    )(input)
}

fn dimension_tag(input: &str) -> IResult<&str, Dimensions> {
    preceded(next(tag("dimensions")), next(dimension))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResultData {
    pub n: usize,
    pub dimensions: Dimensions,
    pub result: ResultType,
}

impl FileParser for ResultData {
    /// Assumes the remaining input contains the data.
    /// Data is either a scalar field or a vector field.
    fn parse_data(input: &str) -> IResult<&str, ResultData> {
        // Parse the dimensions.
        let (input, dimensions) = dimension_tag(input)?;
        // Parse the field data.
        let (input, result) = parse_result(input)?;
        let n = match &result {
            ResultType::Scalar(values) => values.len(),
            ResultType::Vector(values) => values.len(),
            _ => 1,
        };
        // Return the new data structure
        Ok((
            input,
            ResultData {
                n,
                dimensions,
                result,
            },
        ))
    }

    // TODO: this is not correct but needs a larger change to work
    fn file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/points")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        println!("{}\n", dimensions_string(&self.dimensions));
        match &self.result {
            ResultType::UniformScalar(value) => {
                writeln!(file, "internalField   uniform {};", value)?;
                Ok(())
            }
            ResultType::UniformVector(value) => {
                write!(file, "internalField   uniform (;")?;
                write_vector_content(&value, file)?;
                writeln!(file, ");")?;
                Ok(())
            }
            ResultType::Scalar(ref values) => {
                writeln!(file, "internalField   nonuniform List<scalar> ")?;
                write_single_data(values, file)
            }
            ResultType::Vector(ref values) => {
                writeln!(file, "internalField   nonuniform List<vector> ")?;
                write_fixed_witdh_data(values, file)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalar() {
        let input = "
dimensions      [0 2 -2 0 0 0 0];

internalField   nonuniform List<scalar> 
4
(
685.183
685.183
685.184
685.184
)";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 2, -2, 0, 0, 0, 0],
            result: ResultType::Scalar(vec![685.183, 685.183, 685.184, 685.184]),
        };
        let (_, actual_value) = ResultData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_parse_vector() {
        let input = "
dimensions      [0 1 -1 0 0 0 0];

internalField   nonuniform List<vector> 
4
(
(-8.52809e-05 0.00019428 0.00777701)
(-8.36566e-05 0.00019361 0.00802691)
(-8.15522e-05 0.000192606 0.00828979)
(-7.90789e-05 0.000191318 0.00856647)
)";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 1, -1, 0, 0, 0, 0],
            result: ResultType::Vector(vec![
                vec![-8.52809e-05, 0.00019428, 0.00777701],
                vec![-8.36566e-05, 0.00019361, 0.00802691],
                vec![-8.15522e-05, 0.000192606, 0.00828979],
                vec![-7.90789e-05, 0.000191318, 0.00856647],
            ]),
        };
        let (_, actual_value) = ResultData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
