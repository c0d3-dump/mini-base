mod database;
mod parser;
mod tui;

fn main() {
    // tui::run()

    let q = "SELECT * FROM user WHERE email=${email} and password=${password:'1234'}";

    let (_, out) = parser::parse_query(q).unwrap();
    println!("{:#?}", out);

    let qout = parser::replace_variables_in_query(q, out);

    println!("{:#?}", qout);
}
