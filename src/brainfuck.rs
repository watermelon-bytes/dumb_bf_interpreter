use std::io::{self, Read, Write};
use std::ops::{AddAssign, SubAssign};

pub struct BrainfuckInterpreter {
    data: [u8; 1_000],
    pointer: usize,
    // TODO: Consider using a reference or a slice instead of usize, like:
    // alternative_pointer: &u8,
    loop_labels: std::collections::VecDeque<usize>,
}

enum PrimitiveOperation {
    Increment,
    Decrement,
}

enum BasicDirection {
    Right,
    Left,
}

/// Capable of describing any error that can occur during program execution
#[derive(Debug, PartialEq)]
pub enum BrfError {
    PointerUnderflow,
    PointerPastArraySize,
    UnclosedBracket,
    ClosingBracketMissingOpening,
    NonAsciiInput,
    InputOutputOperationFailed(std::io::ErrorKind),
}

impl BrainfuckInterpreter {
    pub fn new() -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            data: [0; 1_000], // TODO: Make it grow dynamically.
            pointer: 0,
            // Use as a queue
            loop_labels: std::collections::VecDeque::new(),
        }
    }

    /// Increments or decrements value of the cell at the data pointer.
    fn manipulate_data(&mut self, op: PrimitiveOperation) -> Result<(), BrfError> {
        if let Some(curr_cell_ref) = self.data.get_mut(self.pointer) {
            *curr_cell_ref = match op {
                PrimitiveOperation::Increment => curr_cell_ref.wrapping_add(1),
                PrimitiveOperation::Decrement => curr_cell_ref.wrapping_sub(1),
            };
            Ok(())
        } else {
            Err(BrfError::PointerPastArraySize)
        }
    }

    fn read_data_cell_at_pointer(&self) -> Result<u8, BrfError> {
        match self.data.get(self.pointer) {
            Some(val) => Ok(*val),
            None => Err(BrfError::PointerPastArraySize),
        }
    }

    /// Moves self.pointer in the specified direction by 1 cell
    /// Handles index underflow and overflow
    fn move_pointer(&mut self, direction: BasicDirection) -> Result<(), BrfError> {
        match direction {
            BasicDirection::Right => {
                self.pointer.add_assign(1);
                if self.pointer >= self.data.len() {
                    return Err(BrfError::PointerPastArraySize);
                }
            }
            BasicDirection::Left => {
                if self.pointer == 0 {
                    return Err(BrfError::PointerUnderflow);
                } else {
                    self.pointer.sub_assign(1);
                }
            }
        }
        Ok(())
    }

    fn input_to_cell(&mut self) -> Result<(), BrfError> {
        let mut buf: [u8; 1] = [0];
        if let Err(e) = std::io::stdin().read_exact(&mut buf) {
            dbg!(&e);
            return Err(BrfError::InputOutputOperationFailed(e.kind()));
        }
        let current_cell = match self.data.get_mut(self.pointer) {
            Some(cell_ref) => cell_ref,
            None => return Err(BrfError::PointerPastArraySize),
        };
        *current_cell = buf[0];
        Ok(())
    }

    /// Writes the character at `data[pointer]` to `io::stdout()`
    fn output_current_character(&self) -> std::io::Result<()> {
        let current_data_cell = self.data.get(self.pointer).unwrap();
        std::io::stdout().write_all(&[*current_data_cell])
    }

    pub fn run(&mut self, source: &str) -> Result<(), BrfError> {
        if !source.is_ascii() {
            return Err(BrfError::NonAsciiInput);
        }
        let mut program_counter: usize = 0;
        while let Some(c) = source.bytes().nth(program_counter) {
            match c {
                // TODO: Consider dynamically extending buffer if pointer goes out of bounds
                b'>' => self.move_pointer(BasicDirection::Right)?,
                b'<' => self.move_pointer(BasicDirection::Left)?,

                b'+' => self.manipulate_data(PrimitiveOperation::Increment)?,
                b'-' => self.manipulate_data(PrimitiveOperation::Decrement)?,

                b'.' => self
                    .output_current_character()
                    .expect("error writing to stdout"),

                b',' => self.input_to_cell()?,

                b'[' => self.loop_labels.push_back(program_counter),

                b']' => {
                    if self.read_data_cell_at_pointer()? != 0 {
                        // If value of current cell is not 0, then jump to the next command after the matching '['.
                        program_counter = match self.loop_labels.back() {
                            Some(new_pc) => *new_pc,
                            None => return Err(BrfError::ClosingBracketMissingOpening),
                        };
                    } else if self.loop_labels.pop_back().is_none() {
                        return Err(BrfError::ClosingBracketMissingOpening);
                    }
                }

                _ => {} // We ignore everything else
            };
            program_counter.add_assign(1);
        }
        println!("\n--- end of program output ---");
        if self.loop_labels.is_empty() {
            Ok(())
        } else {
            Err(BrfError::UnclosedBracket)
        }
    }
}
