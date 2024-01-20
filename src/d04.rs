use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calc_p1() {
  if let Ok(file) = File::open("inputs/d4/test.txt") {
    let mut sum: i32 = 0;
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    for curr_line in lines.iter() {
      println!("{}", curr_line);
      let card: Vec<&str> = curr_line.split(": ").collect();
      let cdata: Vec<&str> = card[1].split(" | ").collect();
      let wns: Vec<i32> = cdata[0].split_whitespace().map(|s: &str| s.parse::<i32>().ok().unwrap()).collect();
      let gns: Vec<i32> = cdata[1].split_whitespace().map(|s: &str| s.parse::<i32>().ok().unwrap()).collect();
      let wgns: Vec<i32> = gns.into_iter().filter(|gn| wns.contains(&gn)).collect();
      if wgns.len() > 0 {
        let val = i32::pow(2, wgns.len() as u32 - 1);
        sum += val;
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", wgns, wgns.len(), wgns.len() - 1, val, sum);
      }
    }
    println!("d04-p1: {}", sum);
  } else {
    eprintln!("Error opening input");
  }
}

