use crate::errors::*;
use crate::simpleloop::SimpleLoop;
use std::collections::HashMap;

const INIT_STACK_SIZE: usize = 2000;

pub struct State {
    stack: Vec<i32>,
    memory: HashMap<String, i32>,
    command_pointer: usize,
    ret: bool,
    loop_instance: Option<SimpleLoop>,
}

impl State {
    pub fn new(loop_instance: Option<SimpleLoop>) -> Self {
        State {
            stack: Vec::with_capacity(INIT_STACK_SIZE),
            memory: HashMap::new(),
            command_pointer: 0,
            ret: false,
            loop_instance,
        }
    }

    pub fn push_to_stack(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop_from_stack<'a>(&mut self, message: &'a str) -> Result<i32, EmptyStackError<'a>> {
        self.stack.pop().ok_or(EmptyStackError::new(message))
    }

    pub fn pop_from_stack_option(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn memory_insert(&mut self, variable_name: String, variable: i32) {
        self.memory.insert(variable_name, variable);
    }

    pub fn memory_get<'a>(
        &self,
        variable_name: &String,
        message: &'a str,
    ) -> Result<i32, UnknownVariableLoadingError<'a>> {
        let value = self
            .memory
            .get(variable_name)
            .ok_or(UnknownVariableLoadingError::new(message))?;
        Ok(*value)
    }

    pub fn get_command_pointer(&self) -> usize {
        self.command_pointer
    }

    pub fn set_command_pointer(&mut self, value: usize) {
        self.command_pointer = value;
    }

    pub fn inc_command_pointer(&mut self) -> usize {
        self.command_pointer += 1;
        self.command_pointer
    }

    pub fn get_ret_status(&self) -> bool {
        self.ret
    }

    pub fn set_ret_status(&mut self, value: bool) {
        self.ret = value;
    }

    pub fn get_loop(&mut self) -> &mut Option<SimpleLoop> {
        &mut self.loop_instance
    }
}
