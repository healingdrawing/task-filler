use crate::{parse_::{Parser, ParserState}, debug::{append_to_file, DEBUG_FILE}};

impl Parser {
  pub fn parse_player(&mut self, input_line: &str){
    if input_line.starts_with("$$$ exec p2"){
      self.player_char = ['s', '$'];
      self.enemy_char = ['a', '@'];
      append_to_file(DEBUG_FILE, "$").expect("Unable to write data");
      self.state = ParserState::GOT_PLAYER;
    }
    else if input_line.starts_with("$$$ exec p1"){
      self.player_char = ['a', '@'];
      self.enemy_char = ['s', '$'];
      append_to_file(DEBUG_FILE, "@").expect("Unable to write data");
      self.state = ParserState::GOT_PLAYER;
    }
  }
  
}