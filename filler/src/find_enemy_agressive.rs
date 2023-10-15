use crate::{parse_::Parser, find_::{Finder, Compas}, debug::{append_to_file, DEBUG, DEBUG_FILE}};

impl Finder {
  pub fn find_enemy_agressive(&mut self, parser: &mut Parser)-> [usize;2] {
    let enemy_char = &parser.enemy_char;
    let mut  enemy_xy = [usize::MAX, usize::MAX];
    let anfield = &parser.anfield;
    
    match self.major {
      Compas::N => {
        enemy_xy = [usize::MAX, 0];/* 0 because of -y is the direction for the enemy in case of N, so +y is the most agressive position for the enemy progress. x is minor in case of N*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == enemy_char[0] || anfield[y][x] == enemy_char[1] { /*the cell is covered by enemy char */
              if y > enemy_xy[1] {
                enemy_xy = [x, y];
              }
            }
          }
        }

      },
      Compas::S => {
        /* keep the initial values of enemy_xy. x is minor in case of S*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == enemy_char[0] || anfield[y][x] == enemy_char[1] { /*the cell is covered by enemy char */
              if y < enemy_xy[1] {
                enemy_xy = [x, y];
              }
            }
          }
        }
      },
      Compas::W => {
        enemy_xy = [0, usize::MAX];/* 0 because of -x is the direction for the enemy in case of W, so +x is the most agressive position for the enemy progress. y is minor in case of W*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == enemy_char[0] || anfield[y][x] == enemy_char[1] { /*the cell is covered by enemy char */
              if x > enemy_xy[0] {
                enemy_xy = [x, y];
              }
            }
          }
        }

      },
      Compas::E => {
        /* keep the initial values of enemy_xy. y is minor in case of E*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == enemy_char[0] || anfield[y][x] == enemy_char[1] { /*the cell is covered by enemy char */
              if x < enemy_xy[0] {
                enemy_xy = [x, y];
              }
            }
          }
        }

      },
      Compas::NW => {
        enemy_xy = [0, 0];/* 0 because of -x is the direction for the enemy in case of NW, so +x is the most agressive position for the enemy progress. 0 because of -y is the direction for the enemy in case of NW, so +y is the most agressive position for the enemy progress.*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == enemy_char[0] || anfield[y][x] == enemy_char[1] { /*the cell is covered by enemy char */
              if x > enemy_xy[0] && y > enemy_xy[1] {
                enemy_xy = [x, y];
              }
              else if x > enemy_xy[0] && y >= enemy_xy[1]
              || x >= enemy_xy[0] && y > enemy_xy[1] { /*at least one of coordinates is more agressively moved */
                //todo: implement
                /*here is myddy place, need some way to compare the distances, between present enemy_xy value and perhaps 0,0 and the new one, before set the new value. not clear */
              }
            }
          }
        }
      },
      Compas::NE => {},
      Compas::SW => {},
      Compas::SE => {},
      Compas::CENTRAL => {append_to_file(DEBUG_FILE, &"fail in find_enemy_agressive. Should never fire".to_string()).expect("Unable to write data")},
    }
    enemy_xy

  }
}