use clap::{Arg, Command};
use interpreter::bytecode::ByteCode;
use std::error::Error;

use interpreter::{parser::*, simpleloop::SimpleLoop, state::State};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Bytecode interpreter")
        .version("1.0")
        .author("Mikhail Mikolaichuk")
        .about(
            "Simple bytecode interpreter.
     Syntax allows to perform basic arithmetic work with variables and loops.",
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Input file with code"),
        )
        .get_matches();

    let file = matches
        .value_of("file")
        .unwrap_or("tests/inputs/example.code");

    let input = std::fs::read_to_string(file)?;
    println!("Code:\n{}", input);
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run()?;

    println!("Result: {:?}", result);
    Ok(())
}
