use crate::file_parser::FileParser;
use crate::foam_structure::{FoamField, FoamStructure};
use crate::parser_base::*;
use indexmap::map::IndexMap;
use nom::multi::fold_many0;
use nom::sequence::pair;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};
use std::io::prelude::*;

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
    pub result: FoamField,
    pub boundary_field: Option<FoamStructure>,
}

impl FileParser for ResultData {
    /// Assumes the remaining input contains the data.
    /// Data is either a scalar field or a vector field.
    fn parse_data(input: &str) -> IResult<&str, ResultData> {
        // Parse the dimensions.
        let (input, dimensions) = dimension_tag(input)?;
        // Parse the field data.
        let (input, result) = preceded(next(tag("internalField")), FoamField::parse)(input)?;
        let n = match &result {
            FoamField::Scalar(values) => values.len(),
            FoamField::Vector(values) => values.len(),
            _ => 1,
        };
        // Parse the boundary field which is sometimes present (in initial conditions for example).
        let (input, boundary_field) = opt(FoamStructure::parse)(input)?;
        // let boundary_field = Some(boundary_field);
        // Return the new data structure
        Ok((
            input,
            ResultData {
                n,
                dimensions,
                result,
                boundary_field,
            },
        ))
    }

    fn default_file_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("unspecified_time/unspecified_field")
    }

    fn write_data(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(file, "{}\n", dimensions_string(&self.dimensions))?;
        write!(file, "internalField   ")?;
        self.result.write(file)?;
        if let Some(boundaries) = &self.boundary_field {
            boundaries.write(file)?;
        }
        Ok(())
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
)
;
";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 2, -2, 0, 0, 0, 0],
            result: FoamField::Scalar(vec![685.183, 685.183, 685.184, 685.184]),
            boundary_field: None,
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
)
;";
        let expected_value = ResultData {
            n: 4,
            dimensions: [0, 1, -1, 0, 0, 0, 0],
            result: FoamField::Vector(vec![
                vec![-8.52809e-05, 0.00019428, 0.00777701],
                vec![-8.36566e-05, 0.00019361, 0.00802691],
                vec![-8.15522e-05, 0.000192606, 0.00828979],
                vec![-7.90789e-05, 0.000191318, 0.00856647],
            ]),
            boundary_field: None,
        };
        let (_, actual_value) = ResultData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
