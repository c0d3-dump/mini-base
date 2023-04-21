use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, multispace1},
    combinator::{peek, value},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    preceded(peek(alpha1), take_until("}"))(input)
}

fn parse_variable(input: &str) -> IResult<&str, &str> {
    delimited(tag("${"), parse_identifier, tag("}"))(input)
}

fn parse_special_character(input: &str) -> IResult<&str, &str> {
    alt((
        tag("*"),
        tag("&"),
        tag("$"),
        tag("?"),
        tag("("),
        tag(")"),
        tag("="),
        tag(">"),
        tag("<"),
        tag(">="),
        tag("<="),
    ))(input)
}

pub fn parse_query(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, variables) = many0(alt((
        parse_variable,
        value("", alphanumeric1),
        value("", parse_special_character),
        value("", multispace1),
    )))(input)?;

    let variables = variables.into_iter().filter(|v| v != &"").collect();

    Ok((input, variables))
}

pub fn replace_variables_in_query(input: &str, variables: Vec<&str>) -> String {
    let mut out = input.to_string();

    variables.into_iter().enumerate().for_each(|(_i, var)| {
        let from = format!("${{{var}}}");

        out = out.replace::<&str>(from.as_ref(), "?");
    });
    out
}
