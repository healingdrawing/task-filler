use crate::parse_::{Parser, ParserState};

#[derive(Debug)]
pub struct Finder {
  pub xy: [usize; 2],
  pub answer: String,
}

impl Finder {
  pub fn new() -> Finder {
    Finder {
      xy: [0, 0],
      answer: String::new(),
    }
  }
  
  pub fn find_answer(&mut self, parser: &mut Parser) {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    let player = &parser.player_char;
    
    // todo: implement find solution here
    //first clean the raw data of the field, find solution and write it to self.answer
    
    self.answer = String::from("2 1\n"); // todo: remove this gap
    
    //clean parser
    parser.reset();
    
  }
  
}
