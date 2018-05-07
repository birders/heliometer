#[derive(Debug)]
pub enum Error {
    MismatchedBrackets,
    IOError(std::io::Error),
    EOF
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

type CellContent = u8;

struct Tape {
    contents: [CellContent; 30000],
}

impl Tape {
    pub fn get_cell(&self, index: usize) -> CellContent {
        self.contents[index]
    }
    pub fn set_cell(&mut self, index: usize, value: CellContent) {
        self.contents[index] = value;
    }
    pub fn increment(&mut self, index: usize) {
        if self.contents[index] == 255 {
            self.contents[index] = 0;
        } else {
            self.contents[index] += 1;
        }
    }
    pub fn decrement(&mut self, index: usize) {
        if self.contents[index] == 0 {
            self.contents[index] = 255;
        } else {
            self.contents[index] -= 1;
        }
    }
    pub fn new() -> Tape {
        Tape {
            contents: [0; 30000],
        }
    }
}

struct State<'s, R: std::io::Read + 's, W: std::io::Write + 's> {
    tape: Tape,
    source: Vec<char>,
    instruction_index: usize,
    data_index: usize,
    input: &'s mut R,
    output: &'s mut W,
    small_buffer: [u8; 1],
}

impl<'s, R: std::io::Read, W: std::io::Write> State<'s, R, W> {
    fn run_single(&mut self) -> Result<bool, Error> {
        match self.source.get(self.instruction_index) {
            Some(chr) => {
                //println!("processing {}", chr);
                let result: Result<bool, Error> = match *chr {
                    '.' => {
                        self.output.write_all(
                            &[self.tape.get_cell(self.data_index)]
                            )?;
                        Ok(true)
                    }
                    ',' => {
                        let count = self.input.read(&mut self.small_buffer)?;
                        if count < 1 {
                            return Err(Error::EOF);
                        }
                        self.tape.set_cell(self.data_index,
                                           self.small_buffer[0]);
                        Ok(true)
                    }
                    '+' => {
                        self.tape.increment(self.data_index);
                        Ok(true)
                    }
                    '-' => {
                        self.tape.decrement(self.data_index);
                        Ok(true)
                    }
                    '<' => {
                        self.data_index -= 1;
                        Ok(true)
                    }
                    '>' => {
                        self.data_index += 1;
                        Ok(true)
                    }
                    '[' => {
                        if self.tape.get_cell(self.data_index) > 0 {
                            Ok(true)
                        } else {
                            let mut nested = 1;
                            loop {
                                self.instruction_index += 1;
                                let chr = match self.source.get(
                                    self.instruction_index
                                    ) {
                                    Some(x) => Ok(x),
                                    None => Err(Error::EOF),
                                }?;
                                if *chr == ']' {
                                    nested -= 1;
                                    if nested < 1 {
                                        break;
                                    }
                                }
                                if *chr == '[' {
                                    nested += 1;
                                }
                            }
                            Ok(true)
                        }
                    }
                    ']' => {
                        if self.tape.get_cell(self.data_index) == 0 {
                            Ok(true)
                        } else {
                            let mut nested = 1;
                            loop {
                                if self.instruction_index < 1 {
                                    return Err(Error::MismatchedBrackets);
                                }
                                self.instruction_index -= 1;
                                let chr = self.source[self.instruction_index];
                                if chr == '[' {
                                    nested -= 1;
                                    if nested < 1 {
                                        break;
                                    }
                                }
                                if chr == ']' {
                                    nested += 1;
                                }
                            }
                            Ok(true)
                        }
                    }
                    _ => Ok(true),
                };
                if result? {
                    self.instruction_index += 1;
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        loop {
            if !self.run_single()? {
                break;
            }
        }
        //self.output.write(&[13]);
        self.output.flush()?;
        Ok(())
    }

    pub fn new<'a>(src: &str, input: &'a mut R, output: &'a mut W)
        -> State<'a, R, W> {
            State {
                tape: Tape::new(),
                source: src.chars().collect(),
                instruction_index: 0,
                data_index: 0,
                input,
                output,
                small_buffer: [0; 1],
            }
        }
}

/// Execute a bf program
///
/// # Example
///
/// ```
/// let input = "+++++[->+++++ +++++<]-- .";
/// heliometer::execute(&input, &mut std::io::stdin(), &mut std::io::stdout()).unwrap();
/// ```
pub fn execute<R: std::io::Read, W: std::io::Write>(
    source: &str,
    input: &mut R,
    output: &mut W,
    ) -> Result<(), Error> {
    let mut state = State::new(source, input, output);
    state.execute()
}
