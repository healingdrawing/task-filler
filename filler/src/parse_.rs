use std::collections::VecDeque;

#[derive(Debug)]
pub enum ParserState {
  WAITING, // fired only once , in the beginning , because engine sends it only once
  GOT_PLAYER,
  GOT_ANFIELD_SIZE,
  GOT_ANFIELD,
  GOT_PIECE_SIZE,
  GOT_PIECE,
}

#[derive(Debug)]
pub struct Parser {
  pub state: ParserState,
  pub player_char: [char; 2],
  pub enemy_char: [char; 2],
  pub anfield_size: [usize; 2],
  pub anfield: VecDeque<VecDeque<char>>,
  pub piece_size: [usize; 2],
  pub piece: VecDeque<VecDeque<char>>,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      state: ParserState::WAITING,
      player_char: ['\0', '\0'],
      enemy_char: ['\0', '\0'],
      anfield_size: [0, 0],
      anfield: VecDeque::new(),
      piece_size: [0, 0],
      piece: VecDeque::new(),
    }
  }

  pub fn parse(&mut self, input_line: &str){
    // append_to_file(DEBUG_FILE, &format!("=== state {:?}=== crap {}", self.state, &input_line)).expect("fail parse"); // todo: remove debug
    match self.state {
      ParserState::WAITING => {self.parse_player(input_line);},
      ParserState::GOT_PLAYER => {self.parse_anfield_size(input_line)},
      ParserState::GOT_ANFIELD_SIZE => {self.parse_anfield(input_line)},
      ParserState::GOT_ANFIELD => {self.parse_piece_size(input_line)},
      ParserState::GOT_PIECE_SIZE => {self.parse_piece(input_line)},
      ParserState::GOT_PIECE => {},
    }
  }

  pub fn reset(&mut self){
    self.state = ParserState::GOT_PLAYER; //skip waiting, it is fired only once
    // self.player_char = ['\0', '\0'];
    self.anfield_size = [0, 0];
    self.anfield = VecDeque::new();
    self.piece_size = [0, 0];
    self.piece = VecDeque::new();
  }

}