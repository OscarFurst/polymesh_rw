use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::count,
    number::complete::double, IResult,
};

/// A point is a 3D coordinate.
type Point = [f64; 3];

/// The PointData structure holds the data of a polyMesh/points file.
#[derive(Debug, PartialEq, Clone)]
pub struct PointData(pub Vec<Point>);

impl std::ops::Deref for PointData {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PointData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FileParser for PointData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/points")
    }
}

fn point_coordinates(input: &str) -> IResult<&str, Point> {
    map(count(ws(double), 3), |v| [v[0], v[1], v[2]])(input)
}
fn point(input: &str) -> IResult<&str, Point> {
    inline_parentheses(point_coordinates)(input)
}

impl FileElement for PointData {
    fn parse(input: &str) -> IResult<&str, PointData> {
        // Parse the number of points.
        let (input, n) = next(digit1)(input)?;
        let n = n.parse().expect("Failed to parse number of points.");
        let (input, _) = next(tag("("))(input)?;
        // Parse exactly this many points.
        let (input, points) = count(next(point), n)(input)?;
        // If the number of points was accurate this schould work:
        let (input, _) = next(tag(")"))(input)?;
        // Return the new data structure
        Ok((input, PointData(points)))
    }
}

impl std::fmt::Display for PointData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_fixed_witdh_data(&self.0, f)
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
        let expected_value = PointData(vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
        ]);
        let (_, actual_value) = PointData::parse(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
