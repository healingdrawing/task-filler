use crate::{parse_::Parser, find_::Finder};

impl Finder {
  pub fn find_player(&mut self, parser: &mut Parser)-> [usize;2] {
    let player = &parser.player_char[1];
    let anfield = &parser.anfield;
    let mut player_xy = [usize::MAX, usize::MAX];
    for (y, row) in anfield.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell == player {
          player_xy = [x, y];
          break;
        }
      }
    }
    player_xy
  }
}