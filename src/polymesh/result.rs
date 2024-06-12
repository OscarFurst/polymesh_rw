use crate::base::{parser_base::*, FileElement};
use crate::base::{FileParser, FoamField, FoamStructure};
use nom::combinator::map;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::opt,
    multi::count,
    sequence::{delimited, preceded},
    IResult,
};
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct ResultData {
    pub n: usize,
    pub dimensions: Dimensions,
    pub result: FoamField,
    pub boundary_field: Option<FoamStructure>,
}

impl FileParser for ResultData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("unspecifiedTimeDirectory/unspecifiedVariableName")
    }
}

impl FileElement for ResultData {
    /// Data is either a scalar field or a vector field.
    fn parse(input: &str) -> IResult<&str, ResultData> {
        // Parse the dimensions.
        let (input, dimensions) = Dimensions::parse(input)?;
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
}

impl std::fmt::Display for ResultData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}\n", &self.dimensions)?;
        write!(f, "internalField   ")?;
        writeln!(f, "{}", &self.result)?;
        if let Some(boundaries) = &self.boundary_field {
            writeln!(f, "{}", boundaries)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions([i32; 7]);

impl Deref for Dimensions {
    type Target = [i32; 7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FileElement for Dimensions {
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(next(tag("dimensions")), next(Self::parse_data))(input)
    }
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dimensions      [{} {} {} {} {} {} {}];",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6]
        )
    }
}

impl Dimensions {
    pub fn new(dimensions: [i32; 7]) -> Self {
        Self(dimensions)
    }

    fn parse_data(input: &str) -> IResult<&str, Self> {
        map(
            delimited(char('['), count(lws(i32_val), 7), tag("];")),
            |x| Dimensions(Vec::try_into(x).unwrap()),
        )(input)
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
            dimensions: Dimensions([0, 2, -2, 0, 0, 0, 0]),
            result: FoamField::Scalar(vec![685.183, 685.183, 685.184, 685.184]),
            boundary_field: None,
        };
        let (_, actual_value) = ResultData::parse(input).unwrap();
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
            dimensions: Dimensions([0, 1, -1, 0, 0, 0, 0]),
            result: FoamField::Vector(vec![
                vec![-8.52809e-05, 0.00019428, 0.00777701],
                vec![-8.36566e-05, 0.00019361, 0.00802691],
                vec![-8.15522e-05, 0.000192606, 0.00828979],
                vec![-7.90789e-05, 0.000191318, 0.00856647],
            ]),
            boundary_field: None,
        };
        let (_, actual_value) = ResultData::parse(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
