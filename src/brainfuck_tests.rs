#[cfg(test)]
use crate::brainfuck::{self, BrfError};

#[test]
fn unclosed_bracket() {
    let res = brainfuck::BrainfuckInterpreter::new().run("[[[ [ ]]]");
    assert_eq!(res, Err(BrfError::UnclosedBracket));
}

#[test]
fn basic_pointer_underflow() {
    assert_eq!(
        brainfuck::BrainfuckInterpreter::new().run("<+"),
        Err(BrfError::PointerUnderflow)
    );
}
