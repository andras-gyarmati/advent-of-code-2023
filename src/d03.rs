use std::io;

use crate::file_reader;

pub fn calc_p1(file_name: &str) -> Result<i32, io::Error> {
  let lines = file_reader::read_lines_from_file(file_name)?;

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

  Ok(sum)
}

#[derive(Copy, Clone)]
struct Gear {
  gear_line_idx: usize,
  gear_char_idx: usize,
  gear_num_cnt: i32,
  gear_num_prod: i32,
}

pub fn calc_p2(file_name: &str) -> Result<i32, io::Error> {
  let lines = file_reader::read_lines_from_file(file_name)?;

  let mut sum = 0;
  let mut nums: Vec<(String, usize, usize)> = Vec::new();
  let mut gears: Vec<Gear> = Vec::new();
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
        if c == '*' {
          // Gear found
          gears.push(Gear { gear_line_idx: line_idx, gear_char_idx: char_idx, gear_num_cnt: 0, gear_num_prod: 1 });
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
    gears.retain(|gear| {
      let keep = gear.gear_line_idx + 1 >= line_idx;
      if !keep {
        // Sum the gears with two numbers
        if gear.gear_num_cnt == 2 {
          sum = sum + gear.gear_num_prod;
        }
      }
      keep
    });

    // Check gears if they are near a number, remove numbers that are summed
    for gear in gears.iter_mut() {
      nums.retain(|(num, _num_line_idx, num_char_idx)| {
        let mut keep = true;
        if *num_char_idx <= gear.gear_char_idx + 1 && gear.gear_char_idx <= num_char_idx + num.len() {
          gear.gear_num_cnt += 1;
          gear.gear_num_prod *= num.parse::<i32>().unwrap_or_default();
          keep = false;
        }
        keep
      });
    }
  }

  Ok(sum)
}
