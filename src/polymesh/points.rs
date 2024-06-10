use crate::base::*;
use crate::file_parser::FileParser;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::count,
    number::complete::double, IResult,
};

type Point = [f64; 3];

#[derive(Debug, PartialEq, Clone)]
pub struct PointData {
    pub n: usize,
    pub points: Vec<Point>,
}

fn point_coordinates(input: &str) -> IResult<&str, Point> {
    map(count(ws(double), 3), |v| [v[0], v[1], v[2]])(input)
}
fn point(input: &str) -> IResult<&str, Point> {
    in_parentheses(point_coordinates)(input)
}

impl FileParser for PointData {
    /// Assumes the remaining input contains the point data.
    fn parse_data(input: &str) -> IResult<&str, PointData> {
        // Parse the number of points.
        let (input, n) = next(digit1)(input)?;
        let n = n.parse().expect("Failed to parse number of points.");
        let (input, _) = next(tag("("))(input)?;
        // Parse exactly this many points.
        let (input, points) = count(next(point), n)(input)?;
        // If the number of points was accurate this schould work:
        let (input, _) = next(tag(")"))(input)?;
        // Return the new data structure
        Ok((input, PointData { n, points }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_points() {
        let input = "
4
(
(0 0 1)
(0 0 0)
(0 1 1)
(0 1 0)
)";
        let expected_value = PointData {
            n: 4,
            points: vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 0.0],
            ],
        };
        let (_, actual_value) = PointData::parse_data(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
