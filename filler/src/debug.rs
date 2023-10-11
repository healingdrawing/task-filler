use std::{fs::{File, OpenOptions}, io::{self, Write}};

pub const DEBUG_FILE: &str = "debug.txt";

pub fn recreate_file(file_path: &str) -> io::Result<()> {
  File::create(file_path)?;
  Ok(())
}

// Function to append data to a file
pub fn append_to_file(file_path: &str, data: &str) -> io::Result<()> {
  let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(file_path)?;

  let mut writer = io::BufWriter::new(file);
  writeln!(writer, "{}", data)?;
  Ok(())
}