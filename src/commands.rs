use crate::errors::*;
use crate::state::State;
pub trait Command: std::fmt::Debug {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError>;
}

#[derive(Debug)]
pub struct LoadValueCommand {
    value: i32,
    _line: String,
}

impl LoadValueCommand {
    pub fn new(value: i32, _line: String) -> Self {
        LoadValueCommand { value, _line }
    }
}

impl Command for LoadValueCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        state.push_to_stack(self.value);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct AddCommand {
    line: String,
}

impl AddCommand {
    pub fn new(line: String) -> Self {
        AddCommand { line }
    }
}

impl Command for AddCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let second = state.pop_from_stack(&self.line)?;
        let first = state.pop_from_stack(&self.line)?;
        let result = first
            .checked_add(second)
            .ok_or(IntegerOverflowError::new(&self.line))?;
        state.push_to_stack(result);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct SubCommand {
    line: String,
}

impl SubCommand {
    pub fn new(line: String) -> Self {
        SubCommand { line }
    }
}

impl Command for SubCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let second = state.pop_from_stack(&self.line)?;
        let first = state.pop_from_stack(&self.line)?;
        let result = first
            .checked_sub(second)
            .ok_or(IntegerOverflowError::new(&self.line))?;
        state.push_to_stack(result);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct MulCommand {
    line: String,
}

impl MulCommand {
    pub fn new(line: String) -> Self {
        MulCommand { line }
    }
}

impl Command for MulCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let second = state.pop_from_stack(&self.line)?;
        let first = state.pop_from_stack(&self.line)?;
        let result = first
            .checked_mul(second)
            .ok_or(IntegerOverflowError::new(&self.line))?;
        state.push_to_stack(result);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct DivCommand {
    line: String,
}

impl DivCommand {
    pub fn new(line: String) -> Self {
        DivCommand { line }
    }
}

impl Command for DivCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let second = state.pop_from_stack(&self.line)?;
        let first = state.pop_from_stack(&self.line)?;
        let result = first
            .checked_div(second)
            .ok_or(IntegerOverflowError::new(&self.line))?;
        state.push_to_stack(result);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct WriteVariableCommand {
    variable_name: String,
    line: String,
}

impl WriteVariableCommand {
    pub fn new(variable_name: String, line: String) -> Self {
        WriteVariableCommand {
            variable_name,
            line,
        }
    }
}

impl Command for WriteVariableCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let variable = state.pop_from_stack(&self.line)?;
        state.memory_insert(self.variable_name.clone(), variable);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReadVariableCommand {
    variable_name: String,
    line: String,
}

impl ReadVariableCommand {
    pub fn new(variable_name: String, line: String) -> Self {
        ReadVariableCommand {
            variable_name,
            line,
        }
    }
}

impl Command for ReadVariableCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let variable = state.memory_get(&self.variable_name, &self.line)?;
        state.push_to_stack(variable);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReturnCommand {
    _line: String,
}

impl ReturnCommand {
    pub fn new(_line: String) -> Self {
        ReturnCommand { _line }
    }
}

impl Command for ReturnCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        state.set_ret_status(true);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct StartLoopCommand {
    amount_of_repeats: i32,
    line: String,
}

impl StartLoopCommand {
    pub fn new(amount_of_repeats: i32, line: String) -> Self {
        StartLoopCommand {
            amount_of_repeats,
            line,
        }
    }
}

impl Command for StartLoopCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let pointer = state.get_command_pointer();
        let simpleloop = match state.get_loop() {
            Some(ref mut simpleloop) => simpleloop,
            None => Err(NoLoopInstanceError)?,
        };

        if simpleloop.is_active() {
            return Err(NestedLoopsError::new(&self.line))?;
        }
        simpleloop.activate(pointer + 1, self.amount_of_repeats);
        state.inc_command_pointer();
        Ok(())
    }
}

#[derive(Debug)]
pub struct EndLoopCommand {
    line: String,
}

impl EndLoopCommand {
    pub fn new(line: String) -> Self {
        EndLoopCommand { line }
    }
}

impl Command for EndLoopCommand {
    fn execute(&self, state: &mut State) -> Result<(), RuntimeError> {
        let simpleloop = match state.get_loop() {
            Some(ref mut simpleloop) => simpleloop,
            None => Err(LoopInitError::new(&self.line))?,
        };

        if !simpleloop.is_active() {
            return Err(LoopInitError::new(&self.line))?;
        }

        let start_pointer = simpleloop.get_start_pointer().unwrap();
        simpleloop.dec_counter();

        if simpleloop.is_counter_eq_zero() {
            simpleloop.desactivate();
            state.inc_command_pointer();
        } else {
            state.set_command_pointer(start_pointer);
        }
        Ok(())
    }
}
