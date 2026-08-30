use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::ops::{AddAssign, SubAssign};

fn main() {
    let passed_args: Vec<String> = env::args().collect();
    if passed_args.len() == 1 {
        todo!("parsing from stdandard input");
    }
    let input_filename = passed_args
        .get(1)
        .expect("A source code file name must be provided.");

    let mut buf = String::new();
    match File::open(input_filename) {
        Err(_) => {
            println!("Errors occured while opening file");
            std::process::exit(2);
        }
        Ok(f) => {
            println!("Successfully read file {input_filename}");
            f
        }
    }
    .read_to_string(&mut buf)
    .expect("Could not read file contents");
    BrainfuckInterpreter::new()
        .execute(buf.as_str())
        .unwrap_or_else(|_| {
            std::process::exit(1);
        });
}

struct BrainfuckInterpreter {
    data: [u8; 10_000],
    pointer: usize,
    // TODO: Consider using a reference or a slice instead of usize, like:
    // alternative_pointer: &u8,
}

enum PrimitiveOperation {
    Increment,
    Decrement,
}

enum BasicDirection {
    Right,
    Left,
}

enum BrainfuckError {
    PointerUnderflow,
    PointerPastArraySize,
    _UnclosedBracket,
}

impl BrainfuckInterpreter {
    fn new() -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            data: [0; 10_000], // TODO: Make it grow dynamically.
            pointer: 0,
        }
    }

    /// Increments or decrements value of the cell at the data pointer.
    fn manipulate_data(&mut self, op: PrimitiveOperation) -> Result<(), BrainfuckError> {
        if let Some(curr_cell_ref) = self.data.get_mut(self.pointer) {
            match op {
                PrimitiveOperation::Increment => curr_cell_ref.add_assign(1),
                PrimitiveOperation::Decrement => curr_cell_ref.sub_assign(1),
            };
            Ok(())
        } else {
            Err(BrainfuckError::PointerPastArraySize)
        }
    }

    fn move_pointer(&mut self, direction: BasicDirection) -> Result<(), BrainfuckError> {
        match direction {
            BasicDirection::Right => {
                if self.pointer > self.data.len() {
                    return Err(BrainfuckError::PointerPastArraySize);
                }
                self.pointer += 1;
            }
            BasicDirection::Left => {
                if self.pointer == 0 {
                    return Err(BrainfuckError::PointerUnderflow);
                }
            }
        }
        Ok(())
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

    fn output_current_character(&self) -> io::Result<()> {
        let current_data_cell = self.data.get(self.pointer).unwrap();
        io::stdout().write_all(&[*current_data_cell])
    }

    fn execute(&mut self, source: &str) -> Result<(), BrainfuckError> {
        for c in source.chars() {
            match c {
                '>' => {
                    self.move_pointer(BasicDirection::Right)?;
                    // TODO: Consider dynamically extending buffer if pointer goes out of bounds
                }
                '<' => {
                    self.move_pointer(BasicDirection::Left)?;
                }
                '+' => {
                    self.manipulate_data(PrimitiveOperation::Increment)?;
                }
                '-' => {
                    self.manipulate_data(PrimitiveOperation::Decrement)?;
                }
                '.' => {
                    self.output_current_character()
                        .expect("error writing to stdout");
                }
                ',' => {
                    self.input_to_cell();
                }
                '[' => todo!("loops via [...]"),
                ']' => todo!("loops via [...]"),
                _ => {} // We ignore everything else
            };
        }
        println!("\n[end of program output]");
        Ok(())
    } // fn execute
}
