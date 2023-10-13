mod debug;
mod parse_;
mod parse_player;
mod parse_anfield;
mod parse_piece;
mod find_;

use debug::{append_to_file, recreate_file, DEBUG_FILE};
use std::io::{self, Write};

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
            println!("{}", finder.answer);
            // println!("3 3\n");
          },
          _ => (),
        }
        
        // // sum_strings.push("\nstep".to_string());
        // sum_strings.push(input_line.clone());
        
        // if input_line.starts_with("P") {
        //   if let Err(err) = append_to_file(DEBUG_FILE, &sum_strings.join("\n")) {
        //     // Handle the error appropriately
        //     eprintln!("Error writing to file: {}", err);
        //     break;
        //   }
        //   println!("2 1\n");
        // }
        
        // Pause for 0.1 seconds
        // thread::sleep(Duration::from_millis(100));
      }
      Err(error) => {
        // Handle the error appropriately
        eprintln!("Error reading input: {}", error);
        break;
      }
    }
  }
  
  // Print the collected values
  // println!("Sum of Strings:\n{}", sum_strings.join("\n"));
}
