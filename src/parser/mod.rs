use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, digit1, multispace1},
    combinator::{peek, value},
    multi::{many0, many1},
    sequence::{delimited, preceded},
    IResult,
};

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    preceded(alt((peek(alpha1), peek(tag(".")))), take_until("}"))(input)
}

fn parse_variable(input: &str) -> IResult<&str, &str> {
    delimited(tag("${"), parse_identifier, tag("}"))(input)
}

fn parse_special_character1(input: &str) -> IResult<&str, &str> {
    alt((
        tag("*"),
        tag("&"),
        tag("|"),
        tag("$"),
        tag("?"),
        tag("("),
        tag(")"),
        tag("="),
        tag(">"),
        tag("<"),
        tag(">="),
        tag("<="),
        tag("_"),
        tag("-"),
        tag(","),
        tag("."),
        tag(";"),
        tag("["),
        tag("]"),
        tag("{"),
        tag("}"),
    ))(input)
}

fn parse_special_character2(input: &str) -> IResult<&str, &str> {
    alt((
        tag("!"),
        tag("#"),
        tag("@"),
        tag("+"),
        tag("/"),
        tag("\\"),
        tag("?"),
        tag(":"),
        tag("'"),
        tag("\""),
        tag("`"),
        tag("~"),
        tag("^"),
    ))(input)
}

pub fn parse_query(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, variables) = many0(alt((
        parse_variable,
        value("", alphanumeric1),
        value("", parse_special_character1),
        value("", parse_special_character2),
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

pub fn parse_type(input: &str) -> &str {
    let x: IResult<&str, &str> = alt((tag("true"), tag("false")))(input);
    match x {
        Ok(_) => "bool",
        Err(_) => {
            let x: IResult<&str, Vec<&str>> = many1(alt((digit1, tag("."))))(input);
            match x {
                Ok(_) => "number",
                Err(_) => {
                    let x: IResult<&str, Vec<&str>> =
                        many1(alt((alphanumeric1, tag(" "), tag("\\n"), tag("\\t"))))(input);
                    match x {
                        Ok(_) => "string",
                        Err(_) => "null",
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_query, parse_type, replace_variables_in_query};

    #[test]
    fn test() {
        assert_eq!(parse_type("1"), "number");
        assert_eq!(parse_type("1.2"), "number");
        assert_eq!(parse_type("true"), "bool");
        assert_eq!(parse_type("false"), "bool");
        assert_eq!(parse_type("Hello "), "string");
    }

    #[test]
    fn test1() {
        assert_eq!(
            parse_query("SELECT * FROM todos where user_id=${userId};"),
            Ok(("", vec!["userId"]))
        )
    }

    #[test]
    fn test2() {
        let query = "SELECT * FROM todos where user_id=${userId};";
        let (_, parmas) = parse_query(query).unwrap();

        assert_eq!(
            replace_variables_in_query(query, parmas),
            String::from("SELECT * FROM todos where user_id=?;")
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse_query("INSERT INTO todos VALUES (${title}, ${isCompleted}, ${.userId});"),
            Ok(("", vec!["title", "isCompleted", ".userId"]))
        )
    }

    #[test]
    fn test4() {
        let query = "INSERT INTO todos VALUES (${title}, ${isCompleted}, ${.userId});";
        let (_, parmas) = parse_query(query).unwrap();

        assert_eq!(
            replace_variables_in_query(query, parmas),
            String::from("INSERT INTO todos VALUES (?, ?, ?);")
        )
    }
}
