use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn read_lines_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let lines = reader.lines().collect::<Result<Vec<_>, io::Error>>();
  lines
}
