use crate::base::parser_base::*;
use crate::base::writer_base::*;
use crate::base::FileElement;
use crate::base::FileParser;
use nom::{bytes::complete::tag, character::complete::digit1, multi::count, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct FaceData {
    pub n: usize,
    // each faces ist just a list of point numbers
    pub faces: Vec<Vec<usize>>,
}

impl FileParser for FaceData {
    fn default_file_path() -> std::path::PathBuf {
        std::path::PathBuf::from("constant/polyMesh/faces")
    }
}

fn parse_face_vertices(input: &str, n: usize) -> IResult<&str, Vec<usize>> {
    count(ws(usize_val), n)(input)
}

fn parse_face(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, n_vertices_str) = digit1(input)?;
    let n_vertices = n_vertices_str
        .parse()
        .expect("Failed to parse number of vertices in face.");
    let (input, vertices) = inline_parentheses(move |i| parse_face_vertices(i, n_vertices))(input)?;
    Ok((input, vertices))
}

impl FileElement for FaceData {
    fn parse(input: &str) -> IResult<&str, FaceData> {
        // Parse the number of faces.
        let (input, n_faces) = next(usize_val)(input)?;
        let (input, _) = next(tag("("))(input)?;
        // Parse exactly this many faces.
        let (input, faces) = count(next(parse_face), n_faces)(input)?;
        // If the number of faces was accurate this schould work:
        let (input, _) = next(tag(")"))(input)?;
        // Return the new data structure
        Ok((input, FaceData { n: n_faces, faces }))
    }

    // fn default_file_path(&self) -> std::path::PathBuf {
    //     std::path::PathBuf::from("constant/polyMesh/faces")
    // }
}

impl std::fmt::Display for FaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_multi_data(&self.faces, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_faces() {
        let input = "
4
(
3(42 92 84)
3(113 84 92)
3(42 84 113)
3(42 113 92)
)";
        let expected_value = FaceData {
            n: 4,
            faces: vec![
                vec![42, 92, 84],
                vec![113, 84, 92],
                vec![42, 84, 113],
                vec![42, 113, 92],
            ],
        };
        let (_, actual_value) = FaceData::parse(input).unwrap();
        assert_eq!(expected_value, actual_value);
    }
}
