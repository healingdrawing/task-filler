
pub enum ParserState {
  WAITING,
  GOT_PLAYER_NAME,
  GOT_ANFIELD_SIZE,
  GOT_ANFIELD,
  GOT_PIECE_SIZE,
  GOT_PIECE,
}

pub struct Parser {
  state: ParserState,
  player_name: String,
  anfield_size: [usize; 2],
  anfield: Vec<Vec<char>>,
  piece_size: [usize; 2],
  piece: Vec<Vec<char>>,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      state: ParserState::WAITING,
      player_name: String::new(),
      anfield_size: [0, 0],
      anfield: Vec::new(),
      piece_size: [0, 0],
      piece: Vec::new(),
    }
  }

  pub fn parse(&mut self, input_line: &str) -> String {
    let mut output = String::new();
    match self.state {
      ParserState::WAITING => {},
      ParserState::GOT_PLAYER_NAME => {},
      ParserState::GOT_ANFIELD_SIZE => {},
      ParserState::GOT_ANFIELD => {},
      ParserState::GOT_PIECE_SIZE => {},
      ParserState::GOT_PIECE => {},
    }
    output
  }
}