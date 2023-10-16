use std::collections::VecDeque;

use crate::{find_::Finder, parse_::Parser, debug::{DEBUG_FILE, append_to_file}};

impl Finder {
  /** todo: find the optimal position for the piece
  * iterate all possible positions for the piece and in time of iteration:
  * 
  * find the most agressively placed enemy cell, opposite the major direction
  * (placed on the player territory, or aimed closer to the player territory)
  *
  * find the most agressively placed player cell, on the major direction
  * (placed on the enemy territory, or aimed closer to the enemy territory)
  *
  * //todo: refactor bottom comments, to possible one iteration for all, with save, probably two possible answers, before choose the best one
  * find all possible variants to place the piece on the field correctly
  *
  * iterate the correct variants
  * check if there is variant to place the piece on the field that some cell of that piece
  * will be placed not the less agressively than the enemy cell, so the same or more agressively
  * from the enemy position. if there is such variant - place the piece there(set the self.answer)
  * to try prevent or restrict the enemy cell to move in the major direction
  *
  * otherwise iterate the correct variants again
  * and place the piece on the field with the most agressively placed player cell, as possible deep
  * in the major direction, to try cover more enemy cells in the major direction
  *
  * otherwise return the surrender answer (the most far enemy position)
  */
  pub fn find_position(&mut self, parser: &mut Parser) -> [usize;2] {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    let player_char = &parser.player_char;
    let enemy_char = &parser.enemy_char;
    
    /* the piece position */
    let mut answer_xy = [usize::MAX, usize::MAX];
    /* the most argessive enemy cell position */
    let agressive_enemy_xy = self.find_agressive(&parser, enemy_char, self.minor);
    let closer_to_agressive_enemy_distance = self.find_cell_min_distance_to_cell_of_opposite_team(anfield, agressive_enemy_xy);
    let mut minimal_to_agressive_enemy_distance = closer_to_agressive_enemy_distance;

    // todo: implement. still not implemented
    /* the most argessive player cell position */
    let agressive_player_xy = self.find_agressive(&parser, player_char, self.major);
    /* the most argessive player cell position on the left side of the major direction */
    let agressive_player_left_xy = self.find_agressive(&parser, player_char, self.major_left);
    /* the most argessive player cell position on the right side of the major direction */
    let agressive_player_right_xy = self.find_agressive(&parser, player_char, self.major_right);
    
    //todo: implement.
    /* to default values for correct piece position */
    let mut fresh_calculation:bool = true;
    
    let piece_height = piece.len();
    let piece_width = piece[0].len();
    
    for y in 0..anfield.len() - piece_height + 1 {
      for x in 0..anfield[0].len() - piece_width + 1 {
        
        if self.position_is_correct(anfield, piece, x, y, player_char){
          //todo: implement.
          /*
            check the most agressively placed enemy cell, opposite the major direction
            check the most agressively placed player cell, on the major direction
            update the answer_xy, if the coordinates satisfy at least one of the conditions above(complete the requirements)
          */
          
          if fresh_calculation {/*use the first found correct position for the piece as default */
            fresh_calculation = false;
            answer_xy = [x, y];
          } else {
            /*first priority to prevent the invasion from the opposite direction */
            let closer_piece_cell_distance_to_agressive_enemy_cell =
            self.find_piece_cell_min_distance_to_cell_xy(
              anfield, piece,
              [x,y],
              agressive_enemy_xy);
            if closer_piece_cell_distance_to_agressive_enemy_cell < minimal_to_agressive_enemy_distance {
              minimal_to_agressive_enemy_distance = closer_piece_cell_distance_to_agressive_enemy_cell;
              answer_xy = [x, y];
            }
            //todo: implement.
            /*
             add check if the minimal_to_agressive_enemy_distance longer than diagonal of the piece etc
             than continue to check the most agressively placed player cell, on the major direction,
             because the enemy cell is not so close to the piece, even if it is the first priority
             And the player way stuff must manage the closest distance to the any enemy cell too,
             before it can replace the first priority calculated answer_xy
            */

            //todo: implement.
            /*also think about add as statement the requirement for piece
            to have some more agressively positioned cell then enemy*/
          }

        }
      }
    }
    append_to_file(DEBUG_FILE, &format!("\n====\nanswer_xy: {} {}", answer_xy[0], answer_xy[1])).expect("cannot write to debug file");
    answer_xy
  }
  
  /** 
  the piece position is correct if all(except one) non-empty cells of the piece
  are placed on the empty cells of the field, and only one non-empty cell
  of the piece is placed on the player cell(any cell covered by the
    player char by the player piece placement previously)
    */
    fn position_is_correct(&self, anfield: &VecDeque<VecDeque<char>>, piece: &VecDeque<VecDeque<char>>, x: usize, y: usize, player:&[char;2]) -> bool {
      
      append_to_file(DEBUG_FILE, &format!("inside ===\nanfield {:?}" ,anfield)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("piece {:?}" ,piece)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("x {} y {}" ,x,y)).expect("cannot write to debug file");
      append_to_file(DEBUG_FILE, &format!("player {:?}" ,player)).expect("cannot write to debug file");
      
      /*
      only one cell from the piece must be placed on the player cell, so
      when the player_cells_hovered_by_piece is 1, for all the piece cells,
      the position is correct, otherwise it is not
      */
      let mut player_cells_hovered_by_piece:usize = 0;
      
      /*iterate the piece and compare the cells with the field cells using the x and y incrementation*/
      for (piece_y, field_y) in (0..piece.len()).zip(y..y + piece.len()) { /*vertical row step */
        for (piece_x, field_x) in (0..piece[0].len()).zip(x..x+piece[0].len()) {/*column */
          if piece[piece_y][piece_x] != '.' {/*if the piece cell is not empty*/
            if anfield[field_y][field_x] != '.' {/*if the field cell is not empty*/
              /* both cells (anfield, piece) are not empty, so need extra check*/
              
              if player_cells_hovered_by_piece > 0{/*if at least one player cell is already hovered by piece*/
                return false;/*the piece position is not correct*/
              }
              
              if anfield[field_y][field_x] == player[0] || anfield[field_y][field_x] == player[1] {/*if the field cell is player cell*/
                player_cells_hovered_by_piece += 1;/*increment the player cells hovered by piece*/
              } else {/*if the field cell is enemy cell*/
                return false;/*the piece position is not correct*/
              }
              
            }
          }
        }
      }
      
      if player_cells_hovered_by_piece == 0 {/*if(finally) no player cell is hovered by piece*/
        return false;/*the piece position is not correct*/
      }
      
      true /*the piece position is correct*/
    }
    
  }