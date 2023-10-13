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
    
    // clean the data

    // build the 2d array , to represent the field

    /*
    [only the first step]
    find the player position
    find the enemy position
    find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
    that is the new piece major direction and save it into finder.
    Later it can be used as way
    to find the enemy cells which are directed to the player size
    and moved already as possible deep in player direction,
    to cut their way first if it is possible.
     */

    // find the most agressively placed enemy cell, opposite the major direction

    // find the most agressively placed player cell, on the major direction

    // find all possible variants to place the piece on the field correctly

    // iterate the correct variants
    // check if there is variant to place the piece on the field that some cell of that piece
    // will be placed not the less agressively than the enemy cell, so the same or more agressively
    // from the enemy position. if there is such variant - place the piece there(set the self.answer)
    // to try prevent or restrict the enemy cell to move in the major direction

    // otherwise iterate the correct variants again
    // and place the piece on the field with the most agressively placed player cell, as possible deep
    // in the major direction, to try cover more enemy cells in the major direction


    self.answer = String::from("2 2"); // todo: remove this gap
    
    //clean parser
    parser.reset();
    
  }
  
}
