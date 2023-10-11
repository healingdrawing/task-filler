use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use std::fs::{OpenOptions, File}; // chatgpt

fn main() {
    recreate_output_file("output.txt").expect("Unable to create file");

    let mut sum_strings = Vec::new();

    loop {
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line) {
            Ok(0) => {
                // EOF (End of File) reached, break the loop
                println!("before print\nInput: {:?}\nafter print", sum_strings);
                io::stdout().flush().unwrap();
                break;
            }
            Ok(_) => {
                let input_line = input_line.trim().to_string();
                
                if input_line.starts_with("$$$"){
                    // println!("1 1\n");
                    append_to_file("output.txt", &"===1 1=== ".to_string()).expect("Unable to write data");
                }

                
                // sum_strings.push("\nstep".to_string());
                sum_strings.push(input_line.clone());
                
                if input_line.starts_with("P"){
                    if let Err(err) = append_to_file("output.txt", &sum_strings.join("\n")) {
                        eprintln!("Error writing to file: {}", err);
                        // Handle the error appropriately
                        // break;
                    }
                }

                // Pause for 0.1 seconds
                thread::sleep(Duration::from_millis(100));
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                // Handle the error appropriately
                // break;
            }
        }
    }

    // Print the collected values
    println!("Sum of Strings:\n{}", sum_strings.join("\n"));
}


fn recreate_output_file(file_path: &str) -> io::Result<()> {
    File::create(file_path)?;
    Ok(())
}

// Function to append data to a file
fn append_to_file(file_path: &str, data: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut writer = io::BufWriter::new(file);
    writeln!(writer, "{}", data)?;
    Ok(())
}