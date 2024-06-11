use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while},
    character::complete::{char, digit0, digit1, multispace0, multispace1},
    combinator::{map, map_res, value},
    multi::{count, many0, many1},
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

/// Modificators

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
// CHECK : the example actually uses "F: 'a" and I dont know why
pub fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes the leading
/// whitespace, returning the output of `inner`.
pub fn lws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(multispace0, inner)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing parentheses on the same line, returning the output of `inner`.
pub fn inline_parentheses<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(char('('), inner, char(')'))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing parentheses, even when on their own lines, returning the output of `inner`.
pub fn block_parentheses<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(next(char('(')), inner, next(char(')')))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes all leading
/// whitespaces and comments.
pub fn next<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    preceded(discard_garbage, inner)
}

/// Parsers

/// A parser that consumes a word, either contained in quotes, or a contiguous word
/// potentially containing /, \, . and _. Returns a `&str`.
pub fn string_val(input: &str) -> IResult<&str, String> {
    let allowed_chars = r#"/\_."#;
    map(
        alt((
            delimited(char('"'), is_not(r#"""#), char('"')),
            take_while(|x| char::is_alphanumeric(x) || allowed_chars.contains(x)),
        )),
        str::to_string,
    )(input)
}

/// A parser that consumes a positive integer and returns it as a `usize`.
pub fn usize_val(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

/// A parser that consumes a "boolean" 0 or 1 and returns it as a `bool`.
pub fn bool(input: &str) -> IResult<&str, bool> {
    map(alt((char('0'), char('1'))), |x| x == '1')(input)
}

pub fn semicolon(input: &str) -> IResult<&str, &str> {
    lws(tag(";"))(input)
}

/// Aggregate parsers

pub fn key_string<'a>(name: &str) -> impl Fn(&'a str) -> IResult<&str, String> + '_ {
    move |i: &'a str| preceded(tag(name), lws(string_val))(i)
}

pub fn key_string_semicolon<'a>(name: &str) -> impl Fn(&'a str) -> IResult<&str, String> + '_ {
    move |i: &'a str| terminated(key_string(name), semicolon)(i)
}

pub fn key_usize<'a>(name: &str) -> impl Fn(&'a str) -> IResult<&str, usize> + '_ {
    move |i: &'a str| preceded(tag(name), lws(usize_val))(i)
}

pub fn key_usize_semicolon<'a>(name: &str) -> impl Fn(&'a str) -> IResult<&str, usize> + '_ {
    move |i: &'a str| terminated(key_usize(name), semicolon)(i)
}

/// A parser that consumes a list of integers, preceded by the integer count, enclosed in parentheses and returns them as a `Vec<usize>`.
/// This format is used for multiple polymesh elements such as owner, neighbour, etc.
pub fn single_i_data(input: &str) -> IResult<&str, Vec<usize>> {
    // data always starts with the number of elements
    let (input, n) = next(usize_val)(input)?;
    // now comes the actual data
    delimited(next(char('(')), count(next(usize_val), n), next(char(')')))(input)
}

/// Parses a list of `double` values separated by whitespace.
pub fn double_values(input: &str) -> IResult<&str, Vec<f64>> {
    many1(lws(double))(input)
}

/// A parser that consumes a list of `double` in parentheses, potentially preceded by the number of entries.
/// This corresponds to the format of vectors, e.g. : (1.0 0.0 0.0)
/// The result is returned as a `Vec<usize>`.
pub fn double_vector(input: &str) -> IResult<&str, Vec<f64>> {
    preceded(digit0, inline_parentheses(double_values))(input)
}

pub fn double_vector_field(input: &str) -> IResult<&str, Vec<Vec<f64>>> {
    // always starts with the number of entries
    let (input, n) = next(usize_val)(input)?;
    // then we have a list of vectors
    delimited(
        next(char('(')),
        count(next(double_vector), n),
        next(char(')')),
    )(input)
}

pub fn double_scalar_field(input: &str) -> IResult<&str, Vec<f64>> {
    // always starts with the number of entries
    let (input, n) = next(usize_val)(input)?;
    // then we have a list of vectors
    delimited(next(char('(')), count(next(double), n), next(char(')')))(input)
}

/// Discarders

pub fn known_key_value<'a, 'b>(
    name: &'b str,
    val: &'b str,
) -> impl Fn(&'a str) -> IResult<&str, ()> + 'b {
    move |i: &'a str| value((), pair(tag(name), lws(tag(val))))(i)
}

pub fn known_key_value_semicolon<'a, 'b>(
    name: &'b str,
    val: &'b str,
) -> impl Fn(&'a str) -> IResult<&str, ()> + 'b {
    |i: &'a str| terminated(known_key_value(name, val), semicolon)(i)
}

pub fn discard_line_comment(input: &str) -> IResult<&str, ()> {
    value((), pair(tag("//"), is_not("\n\r")))(input)
}

pub fn discard_multiline_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(i)
}

pub fn discard_empty(input: &str) -> IResult<&str, ()> {
    value((), multispace1)(input)
}

/// A parser that consumes empty lines and comments.
pub fn discard_garbage(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((
            discard_line_comment,
            discard_multiline_comment,
            discard_empty,
        ))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws() {
        let input = "  hello  ";
        let expected = Ok(("", "hello"));
        let actual = ws(tag("hello"))(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word() {
        let input = "   hello   a";
        let expected = Ok(("a", "hello".to_string()));
        let actual = ws(string_val)(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_large() {
        let input = "
/*--------------------------------*- C++ -*----------------------------------*\\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     | Website:  https://openfoam.org
    \\  /    A nd           | Version:  10
     \\/     M anipulation  |
\\*---------------------------------------------------------------------------*/
FoamFile
{
    format      ascii;
    class       vectorField;
    location    constant/polyMesh;
    object      points;
}
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * //


215";
        let part2 = "
{
    format      ascii;
    class       vectorField;
    location    constant/polyMesh;
    object      points;
}
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * //


215";
        match discard_garbage(input) {
            Ok((i, _)) => {
                let expected = Ok((part2, "FoamFile".to_string()));
                let actual = string_val(i);
                assert_eq!(expected, actual);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);
            }
        }
    }
}
