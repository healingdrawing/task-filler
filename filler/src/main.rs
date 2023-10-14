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

use debug::{append_to_file, recreate_file, DEBUG_FILE};
use std::io::{self};

use crate::parse_::ParserState;

fn main() {
  recreate_file(DEBUG_FILE).expect("Unable to create file");
  
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
            finder.find_answer(&mut parser);
            println!("{}", finder.answer());
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
