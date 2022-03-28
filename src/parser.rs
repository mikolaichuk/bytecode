use crate::{bytecode::CodeType, commands::*, errors::*};

type ParseResult = Result<Option<Box<dyn Command>>, ParserError>;

pub fn parse(input: String) -> Result<CodeType, ParserError> {
    if input.len() == 0 {
        return Err(NoCodeError)?;
    }
    let mut code = CodeType::new();
    let lines = input.lines();

    for line_and_number in lines.enumerate() {
        let result = treat_line_of_code(line_and_number)?;
        match result {
            Some(boxed_command) => code.push(boxed_command),
            None => (),
        }
    }

    Ok(code)
}

fn treat_line_of_code(line_and_number: (usize, &str)) -> ParseResult {
    let (mut line_number, line) = line_and_number;
    line_number += 1;
    let line_with_number = format!("{}:     {}", line_number, line);
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // Line is empty, so nothing to parse
    if tokens.len() == 0 {
        return Ok(None);
    }

    let command_view = tokens.get(0).unwrap();
    let line_with_number_for_args = line_with_number.clone();
    // Check commands without args
    let result: ParseResult = match *command_view {
        "ADD" => Ok(Some(Box::new(AddCommand::new(line_with_number)))),
        "SUB" => Ok(Some(Box::new(SubCommand::new(line_with_number)))),
        "MUL" => Ok(Some(Box::new(MulCommand::new(line_with_number)))),
        "DIV" => Ok(Some(Box::new(DivCommand::new(line_with_number)))),
        "RETURN" => Ok(Some(Box::new(ReturnCommand::new(line_with_number)))),
        "END_LOOP" => Ok(Some(Box::new(EndLoopCommand::new(line_with_number)))),
        _ => Err(ParserError::from(UnknownCommandError::new(
            &line_with_number,
        ))),
    };

    let command_arg = tokens.get(1);
    if command_arg.is_none() {
        return result;
    }
    let argument = *command_arg.unwrap();
    let numerical_argument: Result<i32, _> = argument.parse();
    if numerical_argument.is_ok() {
        // Check commands with numerical arg
        let argument = numerical_argument.unwrap();
        let result: ParseResult = match *command_view {
            "LOAD_VAL" => Ok(Some(Box::new(LoadValueCommand::new(
                argument,
                line_with_number_for_args,
            )))),
            "LOOP" => Ok(Some(Box::new(StartLoopCommand::new(
                argument,
                line_with_number_for_args,
            )))),
            _ => Err(ParserError::from(UnknownCommandError::new(
                &line_with_number_for_args,
            ))),
        };
        return result;
    } else {
        // Check commands with string arg
        let result: ParseResult = match *command_view {
            "WRITE_VAR" => Ok(Some(Box::new(WriteVariableCommand::new(
                argument.to_owned(),
                line_with_number_for_args,
            )))),
            "READ_VAR" => Ok(Some(Box::new(ReadVariableCommand::new(
                argument.to_owned(),
                line_with_number_for_args,
            )))),
            _ => Err(ParserError::from(UnknownCommandError::new(
                &line_with_number_for_args,
            ))),
        };
        return result;
    }
}
