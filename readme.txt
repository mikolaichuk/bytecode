Interpreter allows to work with the following instructions:

ADD - Push two values from the stack, sum them, put result on the stack.
SUB - Push two values from the stack, subtract value2 from value1, where value2 was on the top of the stack, put result on the stack.
MUL - Push two values from the stack, multiply them, put result on the stack.
DIV - Push two values from the stack, divide value1 on value2, where value2 was on the top of the stack, put result on the stack.

WRITE_VAR ARG - Push variable from the top of the stack to the memory. ARG is String which contains variables name.
READ_VAR ARG -  Put variable from the memory on the stack. ARG is String which contains variables name.

LOAD_VAL ARG - Put ARG on the stack. ARG is i32.

RETURN - Finish execution, return Some(value) from the top of the stack or None if stack is empty.
         This command is obligatory.

LOOP ARG - Repeat code between these two instructions ARG times. ARG is i32. Nested loops are forbidden. 
END_LOOP   If only first instruction is presented it will be treated like empty line.

Empty lines are allowed. Each instruction has one or no parameters. 
Extra parameters are ignored. If parameter can not be parsed as i32 it will be treated as String.         

To run unit and integration tests:
cargo test

To run interpreter whith custom code :
    cargo run -- -f path_to_file 

Example (from the root of the project): 
    cargo run -- -f tests/inputs/example.code