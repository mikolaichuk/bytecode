use interpreter::{parser::parse, state::State, simpleloop::SimpleLoop, bytecode::ByteCode};

#[test]
fn integeration_loop_test() {
    let file = "tests/inputs/example_loop.code";
    let input = std::fs::read_to_string(file).unwrap();
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run().unwrap().unwrap();
    assert_eq!(result, 5); 
}

#[test]
fn integration_arithmetic_test() {
    let file = "tests/inputs/example_arithmetic.code";
    let input = std::fs::read_to_string(file).unwrap();
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run().unwrap().unwrap();
    assert_eq!(result, -6); 
}

#[test]
fn integration_example_test() {
    let file = "tests/inputs/example.code";
    let input = std::fs::read_to_string(file).unwrap();
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run().unwrap().unwrap();
    assert_eq!(result, 4); 
}

#[test]
#[should_panic]
fn integration_empty_code_test() {
    let file = "tests/inputs/example_empty.code";
    let input = std::fs::read_to_string(file).unwrap();
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run().unwrap().unwrap();
    assert_eq!(result, 4); 
}

#[test]
#[should_panic]
fn integration_loop_error_test() {
    let file = "tests/inputs/example_loop_error.code";
    let input = std::fs::read_to_string(file).unwrap();
    let result = parse(input).unwrap();
    let simpleloop = SimpleLoop::new();
    let state: State = State::new(Some(simpleloop));
    let mut interpreter = ByteCode::new(state, result);
    let result = interpreter.run().unwrap().unwrap();
    assert_eq!(result, 4); 
}


