use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::cmp;

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

// other idea: start processing character by character and keep track of the one line and one char length of characters if we find a digit we register a new number in a vector and keep processing until we find the end of the number. we have to keep track if the number ends on a line end bc then the number is complete and if the next line start with a digit it's a new number for segments are important: the characters before and after the number and the characters above and below the number plus one character each side of those. if we find a symbol in these segments we have to add the number to the sum. while processing we potentially need to track multiple numbers at the same time until we pass each number's critical segments we have to keep tracking it

pub fn calc_p1_v2() {
  if let Ok(file) = File::open("inputs/d3/test2.txt") {
    let mut sum: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let line_len = lines.get(0).unwrap_or(&String::new()).len();
    let mut num_str = String::new();
    for line in lines.iter() {
      let mut prev_c = '.';
      for c in line.chars() {
        if c.is_numeric() {
          if prev_c.is_numeric() {

          } else {
            match num_str.parse::<i32>() {
              Ok(num) => {
                println!("{}", num);
                nums.push(num);
              }
              Err(_) => {}
            }
            num_str = String::new();
            if !prev_c.eq(&'.') {
              sum += nums.pop().unwrap_or(0);
            }
          }
          num_str.push(c);
          // println!("num_str: {}", num_str);
        } else {
          match num_str.parse::<i32>() {
            Ok(num) => {
              println!("{}", num);
              nums.push(num);
            }
            Err(_) => {}
          }
          num_str = String::new();
          if !c.eq(&'.') {
            if prev_c.is_numeric() {
              sum += nums.pop().unwrap_or(0);
            }
          }
        }
        prev_c = c;
      }
    }
    match num_str.parse::<i32>() {
      Ok(num) => {
        println!("{}", num);
        nums.push(num);
      }
      Err(_) => {}
    }
    println!("d03-p1: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}
