use std::io;

use crate::file_reader;

pub fn calc_p1(file_name: &str) -> Result<i32, io::Error> {
  let lines = file_reader::read_lines_from_file(file_name)?;
  let sum = 0;
    for curr_line in lines.iter() {
      println!("{}", curr_line);
    }

    Ok(sum)
}

