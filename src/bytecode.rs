use crate::{
    commands::*,
    errors::{NoCodeError, NoReturnCommandError, RuntimeError},
    state::State,
};

pub type CodeType = Vec<Box<dyn Command>>;
pub struct ByteCode {
    state: State,
    code: CodeType,
}

impl ByteCode {
    pub fn new(state: State, code: CodeType) -> ByteCode {
        ByteCode { state, code }
    }

    pub fn set_code(&mut self, code: CodeType) {
        self.code = code;
    }

    pub fn run(&mut self) -> Result<Option<i32>, RuntimeError> {
        if self.code.len() == 0 {
            return Err(RuntimeError::from(NoCodeError));
        }
        let mut cmd;
        while {
            cmd = self.code.get(self.state.get_command_pointer());
            cmd.is_some() && !self.state.get_ret_status()
        } {
            cmd.unwrap().execute(&mut self.state)?;
        }

        if !self.state.get_ret_status() {
            return Err(RuntimeError::from(NoReturnCommandError));
        }

        Ok(self.state.pop_from_stack_option())
    }
}

#[cfg(test)]
mod tests {
    use super::super::simpleloop::SimpleLoop;
    use super::*;

    fn get_interpreter() -> ByteCode {
        let simpleloop = SimpleLoop::new();
        let state: State = State::new(Some(simpleloop));
        let code: CodeType = CodeType::new();
        ByteCode::new(state, code)
    }

    #[test]
    fn test_add_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_string())));
        code.push(Box::new(LoadValueCommand::new(1, "".to_string())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 2);
    }

    #[test]
    fn test_sub_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(SubCommand::new(String::from("SUB"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 0);
    }

    #[test]
    fn test_mul_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(4, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(-5, "".to_owned())));
        code.push(Box::new(MulCommand::new(String::from("MUL"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), -20);
    }

    #[test]
    fn test_div_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(10, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(5, "".to_owned())));
        code.push(Box::new(DivCommand::new(String::from("DIV"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 2);
    }

    #[test]
    fn test_load_variable_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(42, "".to_owned())));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 42);
    }

    #[test]
    fn test_read_write_variable_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(42, "".to_owned())));
        code.push(Box::new(WriteVariableCommand::new(
            String::from("x"),
            String::from("WRITE_VAR 'x'"),
        )));
        code.push(Box::new(ReadVariableCommand::new(
            String::from("x"),
            String::from("READ_VAR 'x'"),
        )));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 42);
    }

    #[test]
    fn test_loop_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(StartLoopCommand::new(5, String::from("LOOP 5"))));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(EndLoopCommand::new(String::from("END_LOOP"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn test_no_return_command() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn test_empy_stack_error() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_overflow_error() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(i32::MAX, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(i32::MAX, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_uknown_variable_load_error() {
        let mut code = CodeType::new();
        code.push(Box::new(ReadVariableCommand::new(
            String::from("x"),
            String::from("READ_VAR 'x'"),
        )));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 42);
    }

    #[test]
    #[should_panic]
    fn test_loop_init_error() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(EndLoopCommand::new(String::from("END_LOOP"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn test_nested_loops_error() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(StartLoopCommand::new(5, String::from("LOOP 5"))));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(StartLoopCommand::new(3, String::from("LOOP 3"))));
        code.push(Box::new(EndLoopCommand::new(String::from("END_LOOP"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));

        let mut interpreter = get_interpreter();
        interpreter.set_code(code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn test_no_loop_instance_error() {
        let mut code = CodeType::new();
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(StartLoopCommand::new(5, String::from("LOOP 5"))));
        code.push(Box::new(LoadValueCommand::new(1, "".to_owned())));
        code.push(Box::new(AddCommand::new(String::from("ADD"))));
        code.push(Box::new(StartLoopCommand::new(3, String::from("LOOP 3"))));
        code.push(Box::new(EndLoopCommand::new(String::from("END_LOOP"))));
        code.push(Box::new(ReturnCommand::new("".to_owned())));
        let state: State = State::new(None);
        let mut interpreter = ByteCode::new(state, code);
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }

    #[test]
    #[should_panic]
    fn test_no_code_error() {
        let mut interpreter = get_interpreter();
        assert_eq!(interpreter.run().unwrap().unwrap(), 6);
    }
}
