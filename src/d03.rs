use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::cmp;

// something is off with this one
pub fn calc_p1() {
  if let Ok(file) = File::open("inputs/d3/input.txt") {
    let mut output = File::create("inputs/d3/out.txt").unwrap();
    let mut sum: i32 = 0;
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    for (i, curr_line) in lines.iter().enumerate() {
      let mut prev_line = String::new();
      if i > 0 { prev_line = lines.get(i - 1).unwrap_or(&String::new()).to_string(); }
      let next_line = lines.get(i + 1).unwrap_or(&String::new()).to_string();
      let nums = curr_line.split(|c: char| !c.is_numeric()).filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
      for num in nums {
        let num_str = num.to_string();
        let pos: i32 = curr_line.find(num_str.as_str()).unwrap() as i32;
        let start: usize = cmp::max(0i32, pos - 1) as usize;
        let end: usize = cmp::min(curr_line.len(), pos as usize + num_str.len() + 1);
        let prev: &str = prev_line.get(start..end).unwrap_or("");
        let curr: &str = curr_line.get(start..end).unwrap_or("");
        let next: &str = next_line.get(start..end).unwrap_or("");
        if prev.chars().any(|c| !c.is_alphanumeric() && !c.eq(&'.')) ||
          curr.chars().any(|c| !c.is_alphanumeric() && !c.eq(&'.')) ||
          next.chars().any(|c| !c.is_alphanumeric() && !c.eq(&'.')) {
          sum += num;
          output.write_all(format!("{} {} {} {}\n", num, prev, curr, next).as_bytes()).unwrap();
        }
      }
    }
    println!("d03-p1: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}

pub fn calc_p1_v2() {
  if let Ok(file) = File::open("inputs/d3/input.txt") {
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut sum = 0;
    let mut nums: Vec<(String, usize, usize)> = Vec::new();
    let mut specials: Vec<(char, usize, usize)> = Vec::new();
    let mut current_num = String::new();
    let mut num_start_idx: usize = 0;
    let mut num_line_idx: usize = 0;
    let mut prev_char = '.';

    for (line_idx, line) in lines.iter().enumerate() {
      for (char_idx, c) in line.chars().enumerate() {
        if c.is_numeric() {
          if !prev_char.is_numeric() {
            // Start of a new number
            num_start_idx = char_idx;
            num_line_idx = line_idx;
            current_num.clear();
          }
          current_num.push(c);
        } else {
          if !current_num.is_empty() {
            // End of the current number, save it
            nums.push((current_num.clone(), num_line_idx, num_start_idx));
            current_num.clear();
          }
          if !c.is_alphanumeric() && c != '.' {
            // Special character found
            specials.push((c, line_idx, char_idx));
          }
        }
        prev_char = c;
      }

      if !current_num.is_empty() {
        // End of the line, save the current number if it's not empty
        nums.push((current_num.clone(), num_line_idx, num_start_idx));
        current_num.clear();
      }

      // Remove items if they are in a line before the previous line
      nums.retain(|(_, num_line_idx, _)| {
        *num_line_idx + 1 >= line_idx
      });
      specials.retain(|(_, num_line_idx, _)| {
        *num_line_idx + 1 >= line_idx
      });

      // Check special chars if they are near a number, remove numbers that are summed
      for (_spec, _spec_line_idx, spec_char_idx) in specials.iter() {
        nums.retain(|(num, _num_line_idx, num_char_idx)| {
          let mut keep = true;
          if *num_char_idx <= spec_char_idx + 1 && *spec_char_idx <= num_char_idx + num.len() {
            sum = sum + num.parse::<i32>().unwrap_or_default();
            keep = false;
          }
          keep
        });
      }
    }

    println!("d03-p1 v2: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}
