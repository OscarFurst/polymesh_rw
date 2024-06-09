use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map_res, opt, value},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn in_parentheses<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(char('('), inner, char(')'))
}

pub fn next<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    preceded(discard_garbage, inner)
}

// TODO: This needs to be improved to support keys with spaces when enclosed in quotes.
pub fn key(input: &str) -> IResult<&str, &str> {
    let allowed_chars = r#"/\_"#;
    delimited(
        opt(tag(r#"""#)),
        take_while(|x| char::is_alphanumeric(x) || allowed_chars.contains(x)),
        opt(tag(r#"""#)),
    )(input)
}

pub fn i_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

pub fn semicolon(input: &str) -> IResult<&str, &str> {
    tag(";")(input)
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
        let expected = Ok(("a", "hello"));
        let actual = ws(key)(input);
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
                let expected = Ok((part2, "FoamFile"));
                let actual = key(i);
                assert_eq!(expected, actual);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);
            }
        }
    }
}
