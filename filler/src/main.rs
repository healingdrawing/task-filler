mod debug;
mod parse_;
mod parse_player;
mod parse_anfield;
mod parse_piece;
mod find_;
mod find_player;
mod find_enemy;
mod find_distance;
mod find_direction;
mod find_position;
mod find_agressive;
mod find_diagonal;
mod find_more_agressive;
mod find_most_far;

use debug::{append_to_file, try_recreate_file_according_to_value_of_debug_boolean, DEBUG_FILE};
use std::{io::{self}, thread, time::Duration};

use crate::parse_::ParserState;

fn main() {
  try_recreate_file_according_to_value_of_debug_boolean(DEBUG_FILE).expect("Unable to create file");
  
  let mut parser = parse_::Parser::new();
  let mut finder = find_::Finder::new();
  
  // let mut sum_strings = Vec::new();
  loop {
    let mut input_line = String::new();
    match io::stdin().read_line(&mut input_line) {
      Ok(0) => {
        // EOF (End of File) reached, break the loop
        append_to_file(DEBUG_FILE, "EOF block\n").expect("fail to print inside EOF block");
        break;
      }
      Ok(_) => {
        let input_line = input_line.trim().to_string();
        
        parser.parse(&input_line);

        match parser.state {
          ParserState::GOT_PIECE => {
            let rust_crap_answer = finder.find_answer(&mut parser);
            // thread::sleep(Duration::from_millis(50));

            println!("{} {}",
              rust_crap_answer[0],
              rust_crap_answer[1],
            );

            // todo: sometimes the engine recognize the second line as second response and fails the game. So not desirable to print the second line
            // println!("{} {}\n negative_x -{} negative_y -{}",
            //   rust_crap_answer[0],
            //   rust_crap_answer[1],
            //   finder.piece_negative_xy[0],
            //   finder.piece_negative_xy[1],
            // );
            
            finder.reset_negative_xy();

            // println!("{}", finder.answer());
          },
          _ => (),
        }
        
      }
      Err(error) => {
        // Handle the error appropriately
        eprintln!("Error reading input: {}", error);
        break;
      }

    }
  }
  
}
