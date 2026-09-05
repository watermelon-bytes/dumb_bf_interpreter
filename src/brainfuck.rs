use std::io::{Read, Write};
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
pub enum BrainfuckError {
    PointerUnderflow,
    PointerPastArraySize,
    UnclosedBracket,
    ClosingBracketMissingOpening,
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
    fn manipulate_data(&mut self, op: PrimitiveOperation) -> Result<(), BrainfuckError> {
        if let Some(curr_cell_ref) = self.data.get_mut(self.pointer) {
            *curr_cell_ref = match op {
                PrimitiveOperation::Increment => curr_cell_ref.wrapping_add(1),
                PrimitiveOperation::Decrement => curr_cell_ref.wrapping_sub(1),
            };
            Ok(())
        } else {
            Err(BrainfuckError::PointerPastArraySize)
        }
    }

    fn read_data_cell_at_pointer(&self) -> Result<u8, BrainfuckError> {
        match self.data.get(self.pointer) {
            Some(val) => Ok(*val),
            None => Err(BrainfuckError::PointerPastArraySize),
        }
    }

    fn move_pointer(&mut self, direction: BasicDirection) -> Result<(), BrainfuckError> {
        match direction {
            BasicDirection::Right => {
                self.pointer.add_assign(1);
                if self.pointer > self.data.len() {
                    Err(BrainfuckError::PointerPastArraySize)
                } else {
                    Ok(())
                }
            }
            BasicDirection::Left => {
                if self.pointer == 0 {
                    Err(BrainfuckError::PointerUnderflow)
                } else {
                    self.pointer.sub_assign(1);
                    Ok(())
                }
            }
        }
    }

    // TODO: Return some kind of Result
    fn input_to_cell(&mut self) {
        let mut buf: [u8; 1] = [0];
        if let Err(e) = std::io::stdin().read_exact(&mut buf) {
            dbg!(e);
            panic!("error while reading stdin");
        }
        let current_cell = self
            .data
            .get_mut(self.pointer)
            .expect("Pointer out of bounds");
        *current_cell = buf[0];
    }

    /// Writes the character at `data[pointer]` to `io::stdout()`
    fn output_current_character(&self) -> std::io::Result<()> {
        let current_data_cell = self.data.get(self.pointer).unwrap();
        std::io::stdout().write_all(&[*current_data_cell])
    }

    pub fn run(&mut self, source: &str) -> Result<(), BrainfuckError> {
        let mut program_counter: usize = 0;
        while let Some(c) = source.chars().nth(program_counter) {
            match c {
                // TODO: Consider dynamically extending buffer if pointer goes out of bounds
                '>' => self.move_pointer(BasicDirection::Right)?,
                '<' => self.move_pointer(BasicDirection::Left)?,

                '+' => self.manipulate_data(PrimitiveOperation::Increment)?,
                '-' => self.manipulate_data(PrimitiveOperation::Decrement)?,

                '.' => self
                    .output_current_character()
                    .expect("error writing to stdout"),

                ',' => self.input_to_cell(),

                '[' => self.loop_labels.push_back(program_counter),

                ']' => {
                    if self.read_data_cell_at_pointer()? != 0 {
                        // If value of current cell is not 0, then jump to the next command after the matching '['.
                        program_counter = match self.loop_labels.back() {
                            Some(new_pc) => *new_pc,
                            None => return Err(BrainfuckError::ClosingBracketMissingOpening),
                        };
                    } else if self.loop_labels.pop_back().is_none() {
                        return Err(BrainfuckError::ClosingBracketMissingOpening);
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
            Err(BrainfuckError::UnclosedBracket)
        }
    }
}
