use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calc_p1() {
  if let Ok(file) = File::open("inputs/d1/test.txt") {
    let reader = BufReader::new(file);
    let sum: i32 = reader
      .lines()
      .filter_map(Result::ok)
      .map(|line| {
        let chars = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
        if let (Some(first), Some(last)) = (chars.chars().nth(0), chars.chars().last()) {
          format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
        } else {
          0
        }
      })
      .sum();

    println!("d01-p1: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}

pub fn calc_p2() {
  let spelled_digits = vec! {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
  if let Ok(file) = File::open("inputs/d1/test2.txt") {
    let reader = BufReader::new(file);
    let sum: i32 = reader
      .lines()
      .filter_map(Result::ok)
      .map(|line| {
        // println!("{}", line);
        let mut poses: Vec<(usize, usize)> = vec! {};
        spelled_digits.iter().enumerate().for_each(|(i, &d)| {
          if let Some(idx) = line.find(d) {
            poses.push((idx, i + 1));
          }
          if let Some(idx) = line.find((i + 1).to_string().as_str()) {
            poses.push((idx, i + 1));
          }
          if let Some(idx) = line.rfind(d) {
            poses.push((idx, i + 1));
          }
          if let Some(idx) = line.rfind((i + 1).to_string().as_str()) {
            poses.push((idx, i + 1));
          }
        });
        poses.sort_by_key(|&(fst, _)| fst);
        // println!("{:?}", poses);
        if let (Some((_, first)), Some((_, last))) = (poses.first(), poses.last()) {
          let num = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
          // println!("{} => {}", line, num.to_string());
          num
        } else {
          0
        }
      })
      .sum();

    println!("d01-p2: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}
