use crate::{parse_::Parser, find_::Finder};

impl Finder {
  pub fn find_enemy(&mut self, parser: &mut Parser, player_xy:[usize;2])-> [usize;2] {
    let player = &parser.player_char[1];
    let enemy_char = if player == &'$' {['@', 'a']} else {['$', 's']};
    let anfield = &parser.anfield;
    let mut enemy_xy = player_xy.clone();
    // find the most far enemy cell, to determine the initial major direction
    for (y, row) in anfield.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell == &enemy_char[0] || cell == &enemy_char[1] {
          if self.find_distance(player_xy.clone(), [x, y].clone()) > self.find_distance(player_xy.clone(), enemy_xy.clone()) {
            enemy_xy = [x, y].clone();
          }
        }
      }
    }
    enemy_xy
  }

}