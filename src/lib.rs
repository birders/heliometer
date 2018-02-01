#[derive(Debug)]
pub enum Error {
  MismatchedBrackets,
  InvalidInstruction
}

type CellContent = u8;

struct Tape {
  contents: [CellContent; 30000]
}

impl Tape {
  pub fn get_cell(&self, index: usize) -> CellContent {
    return self.contents[index];
  }
  pub fn increment(&mut self, index: usize) {
    self.contents[index] += 1;
  }
  pub fn decrement(&mut self, index: usize) {
    self.contents[index] -= 1;
  }
  pub fn new() -> Tape {
    Tape {
      contents: [0; 30000]
    }
  }
}

struct State<R: std::io::Read, W: std::io::Write> {
  tape: Tape,
  source: Vec<char>,
  instruction_index: usize,
  data_index: usize,
  input: R,
  output: W
}

impl<R: std::io::Read, W: std::io::Write> State<R, W> {
  fn run_single(&mut self) -> Result<bool, Error> {
    match self.source.get(self.instruction_index) {
      Some(chr) => {
        if match chr {
          &'.' => {
            self.output.write(&[self.tape.get_cell(self.data_index)]);
            Ok(true)
          },
          &'+' => {
            self.tape.increment(self.data_index);
            Ok(true)
          },
          &'-' => {
            self.tape.decrement(self.data_index);
            Ok(true)
          },
          &'<' => {
              self.data_index -= 1;
              Ok(true)
          },
          &'>' => {
              self.data_index += 1;
              Ok(true)
          },
          _ => Err(Error::InvalidInstruction)
        }? {
          self.instruction_index += 1;
        }
        Ok(true)
      },
      None => Ok(false)
    }
  }
  pub fn execute(&mut self) -> Result<(), Error> {
    while true {
      if !self.run_single()? {
        break;
      }
    }
    //self.output.write(&[13]);
    self.output.flush();
    Ok(())
  }
  pub fn new(src: &str, input: R, output: W) -> State<R, W> {
    State {
      tape: Tape::new(),
      source: src.chars().collect(),
      instruction_index: 0,
      data_index: 0,
      input,
      output
    }
  }
}

pub fn execute<R: std::io::Read, W: std::io::Write>(source: &str, input: R, output: W) -> Result<(), Error> {
  let mut state = State::new(source, input, output);
  state.execute()
}


/*
  [x]  >	increment the data pointer (to point to the next cell to the right).
  [x]  <	decrement the data pointer (to point to the next cell to the left).
  [x]  +	increment (increase by one) the byte at the data pointer.
  [x]  -	decrement (decrease by one) the byte at the data pointer.
  [x]  .	output the byte at the data pointer.
  [ ]  ,	accept one byte of input, storing its value in the byte at the data pointer.
  [ ]  [	if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
  [ ]  ]	if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
*/
