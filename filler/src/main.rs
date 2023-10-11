mod debug;

use debug::{recreate_file, append_to_file, DEBUG_FILE};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    recreate_file(DEBUG_FILE).expect("Unable to create file");
    let mut sum_strings = Vec::new();
    loop {
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(0) => {// EOF (End of File) reached, break the loop
                io::stdout().flush().unwrap(); break;
            }
            Ok(_) => {
                let input_line = input_line.trim().to_string();
                
                if input_line.starts_with("$$$"){
                    // println!("1 1\n");
                    append_to_file(DEBUG_FILE, &"===1 1=== ".to_string()).expect("Unable to write data");
                }

                
                // sum_strings.push("\nstep".to_string());
                sum_strings.push(input_line.clone());
                
                if input_line.starts_with("P"){
                    if let Err(err) = append_to_file(DEBUG_FILE, &sum_strings.join("\n")) {// Handle the error appropriately
                        eprintln!("Error writing to file: {}", err); break;
                    }
                    println!("1 1\n");
                }

                // Pause for 0.1 seconds
                // thread::sleep(Duration::from_millis(100));
            }
            Err(error) => { // Handle the error appropriately
                eprintln!("Error reading input: {}", error); break;
            }
        }
    }

    // Print the collected values
    println!("Sum of Strings:\n{}", sum_strings.join("\n"));
}


