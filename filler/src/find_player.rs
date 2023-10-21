use crate::{parse_::Parser, find_::Finder};

impl Finder {
  pub fn find_player(&mut self, parser: &mut Parser)-> [usize;2] {
    let player_0 = &parser.player_char[0].clone();
    let player_1 = &parser.player_char[1].clone();
    /*
      yep, it ugly, but the crap happens,
      so i need to decrease all possible ways to have it
    */
    let anfield = &parser.anfield;
    let mut player_xy = [usize::MAX, usize::MAX];
    for (y, row) in anfield.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell == player_0 || cell == player_1 {
          player_xy = [x, y].clone();
          break;
        }
      }
    }
    player_xy
  }
}