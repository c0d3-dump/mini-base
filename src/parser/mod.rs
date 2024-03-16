use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{alpha1, alphanumeric1, digit1, multispace1},
    combinator::{peek, value},
    multi::{many0, many1},
    sequence::{delimited, preceded},
    IResult,
};

use crate::database::model::ColType;

pub mod sql_parser;

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

pub fn replace_variables_with_values(
    input: &str,
    values: HashMap<String, Option<ColType>>,
) -> String {
    let mut out = input.to_string();

    let binding = out.clone();
    let variables = many0(take_variables)(&binding).unwrap().1;

    for variable in variables {
        let mut t_variable = variable;
        if variable.contains("res.") {
            t_variable = "res";
        }

        if let Some(var) = values.get(t_variable) {
            out = replace_coltype_value(out, var.clone(), variable.split(".").collect(), variable);
        }
    }
    out
}

fn replace_coltype_value(
    out: String,
    variable: Option<ColType>,
    mut res_var: Vec<&str>,
    original_var: &str,
) -> String {
    let from = format!("${{{original_var}}}");

    match variable {
        Some(var) => match var {
            ColType::Integer(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::Real(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::UnsignedInteger(t) => {
                out.replace::<&str>(from.as_ref(), &t.unwrap().to_string())
            }
            ColType::String(t) => {
                let nt = t.unwrap();
                out.replace::<&str>(from.as_ref(), &format!("\"{nt}\""))
            }
            ColType::Bool(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::Date(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::Time(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::Datetime(t) => out.replace::<&str>(from.as_ref(), &t.unwrap().to_string()),
            ColType::Array(t) => {
                res_var.remove(0);
                let rvf = res_var.first().unwrap();
                if let Ok(i) = rvf.parse::<usize>() {
                    let t = t.unwrap();
                    let l = t.get(i).unwrap();

                    replace_coltype_value(out, Some(l.clone()), res_var, original_var)
                } else {
                    "".to_string()
                }
            }
            ColType::Object(t) => {
                res_var.remove(0);
                let rvf = res_var.first().unwrap();
                let t = t.unwrap();
                let l = t.get(*rvf).unwrap();

                replace_coltype_value(out, Some(*l.clone()), res_var, original_var)
            }
            ColType::Json(_) => out.clone(),
        },
        None => out,
    }
}

#[test]
fn test_replace_variables_with_values() {
    let out = "
        {  \"header\": {   \"test\": ${.USER_EMAIL},   \"res\": ${res.0.name}  },  \"body\": {   \"fname\": \"bhavin\",   \"lname\": \"sojitra\",   \"id\": ${.USER_ID},   \"role\": ${.USER_ROLE}  },  \"query\": {   \"ok\": \"there\",   \"roleId\": ${roleId}  } }
    ";

    let mut values = HashMap::new();
    let mut d1 = HashMap::new();
    d1.insert(
        "name".to_string(),
        Box::new(ColType::String(Some("bhavin sojitra".to_string()))),
    );
    let a1 = vec![ColType::Object(Some(d1))];

    values.insert("res".to_string(), Some(ColType::Array(Some(a1))));
    values.insert("roleId".to_string(), Some(ColType::Integer(Some(1))));

    let res = replace_variables_with_values(out, values);

    dbg!(res);
}

fn take_variables(input: &str) -> IResult<&str, &str> {
    delimited(
        preceded(take_until("${"), take(2usize)),
        take_until("}"),
        tag("}"),
    )(input)
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
