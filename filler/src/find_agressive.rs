use crate::{parse_::Parser, find_::{Finder, Compas}, debug::{append_to_file, DEBUG_FILE}};

impl Finder {
  /**
   * find the most agressive position of the target (player or enemy)
   * 
   */
  pub fn find_most_agressive_xy(&mut self, parser: &&mut Parser, target_char:&[char;2], direction_to_target:Compas)-> [usize;2] {
    let anfield = &parser.anfield;
    let mut target_xy = [anfield[0].len()-1, anfield.len()-1];
    
    match direction_to_target {
      Compas::N => {
        target_xy[1] = 0;/* 0 because of -y is the direction for the enemy in case of N, so +y is the most agressive position for the enemy progress. x is minor in case of N*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if y > target_xy[1] {
                target_xy = [x, y];
              }
            }
          }
        }

      },
      Compas::S => {
        /* keep the initial values of enemy_xy. x is minor in case of S*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if y < target_xy[1] {
                target_xy = [x, y].clone();
              }
            }
          }
        }
      },
      Compas::W => {
        target_xy[0] = 0;/* 0 because of -x is the direction for the enemy in case of W, so +x is the most agressive position for the enemy progress. y is minor in case of W*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x > target_xy[0] {
                target_xy = [x, y].clone();
              }
            }
          }
        }

      },
      Compas::E => {
        /* keep the initial values of enemy_xy. y is minor in case of E*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x < target_xy[0] {
                target_xy = [x, y].clone();
              }
            }
          }
        }

      },
      Compas::NW => {
        target_xy = [0, 0].clone();/* 0 because of -x is the direction for the enemy in case of NW, so +x is the most agressive position for the enemy progress. 0 because of -y is the direction for the enemy in case of NW, so +y is the most agressive position for the enemy progress.*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x > target_xy[0] && y > target_xy[1] {
                target_xy = [x, y].clone();
              }
              else if x > target_xy[0] && y >= target_xy[1]
              || x >= target_xy[0] && y > target_xy[1] { /*at least one of coordinates is more agressively moved */
                /*here is myddy place, need some way to compare the distances, between present enemy_xy value and perhaps 0,0 and the new one, before set the new value. not clear */
                if self.find_distance([0,0], [x,y]) > self.find_distance([0,0], target_xy) {
                  target_xy = [x, y].clone();
                }
              }
            }
          }
        }
      },
      Compas::NE => {
        target_xy = [anfield[0].len()-1, 0].clone();/* 0 because of -y is the direction for the enemy in case of NE, so +y is the most agressive position for the enemy progress. anfield[0].len()-1 because of +x is the direction for the enemy in case of NE, so -x is the most agressive position for the enemy progress.*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x < target_xy[0] && y > target_xy[1] {
                target_xy = [x, y].clone();
              } else if x <= target_xy[0] && y > target_xy[1]
              || x < target_xy[0] && y >= target_xy[1] { /*at least one of coordinates is more agressively moved */
                if self.find_distance([anfield[0].len()-1,0], [x,y]) > self.find_distance([anfield[0].len()-1,0], target_xy) {
                  target_xy = [x, y].clone();
                }
              }
            }
          }
        }
      },
      Compas::SW => {
        target_xy = [0, anfield.len()-1].clone();/* anfield.len()-1 because of +y is the direction for the enemy in case of SW, so -y is the most agressive position for the enemy progress. 0 because of -x is the direction for the enemy in case of SW, so +x is the most agressive position for the enemy progress.*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x > target_xy[0] && y < target_xy[1] {
                target_xy = [x, y].clone();
              } else if x > target_xy[0] && y <= target_xy[1]
              || x >= target_xy[0] && y < target_xy[1] { /*at least one of coordinates is more agressively moved */
                if self.find_distance([0,anfield.len()-1], [x,y]) > self.find_distance([0,anfield.len()-1], target_xy) {
                  target_xy = [x, y].clone();
                }
              }
            }
          }
        }
      },
      Compas::SE => {
        target_xy = [anfield[0].len()-1, anfield.len()-1].clone();/* anfield.len()-1 because of +y is the direction for the enemy in case of SE, so -y is the most agressive position for the enemy progress. anfield[0].len()-1 because of +x is the direction for the enemy in case of SE, so -x is the most agressive position for the enemy progress.*/
        for y in 0..anfield.len() {
          for x in 0..anfield[0].len() {
            if anfield[y][x] == target_char[0] || anfield[y][x] == target_char[1] { /*the cell is covered by enemy char */
              if x < target_xy[0] && y < target_xy[1] {
                target_xy = [x, y].clone();
              } else if x <= target_xy[0] && y < target_xy[1]
              || x < target_xy[0] && y <= target_xy[1] { /*at least one of coordinates is more agressively moved */
                if self.find_distance([anfield[0].len()-1,anfield.len()-1], [x,y]) > self.find_distance([anfield[0].len()-1,anfield.len()-1], target_xy) {
                  target_xy = [x, y].clone();
                }
              }
            }
          }
        }
      },
      Compas::CENTRAL => {append_to_file(DEBUG_FILE, &"fail in find_enemy_agressive. Should never fire".to_string()).expect("Unable to write data")},
    }
    target_xy

  }
}