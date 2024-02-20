use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calc_p1() {
  if let Ok(file) = File::open("inputs/d5/test.txt") {
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    for curr_line in lines.iter() {
      println!("{}", curr_line);
    }
  } else {
    eprintln!("Error opening input");
  }
}

