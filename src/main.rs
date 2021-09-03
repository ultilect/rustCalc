use crate::parsemath::parser::ParseError;

mod parsemath;

fn main() {
  println!("Hello! Welcome to my Rust App!");

  loop {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
      Ok(_) => {
        match evaluate(input) {
          Ok(val) => println!("The answer iiis: {}\n",val),
          Err(_) => println!("Error :("),
        }
      }
      Err(error) => println!("Error: {}", error),
    }
  }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
  let expr = expr.split_whitespace().collect::<String>();

  let mut math_parser = parsemath::parser::Parser::new(&expr)?;
  let ast = math_parser.parse()?;
  println!("The generated ast is {:?}", ast);

  Ok(parsemath::ast::eval(ast)?)
}

