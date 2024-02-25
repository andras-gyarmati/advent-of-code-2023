use std::io;

use crate::file_reader;

pub fn calc_p1(file_name: &str) -> Result<i32, io::Error> {
  let lines = file_reader::read_lines_from_file(file_name)?;

  let mut sum: i32 = 0;
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

  Ok(sum)
}

pub fn calc_p2(file_name: &str) -> Result<i32, io::Error> {
  let lines = file_reader::read_lines_from_file(file_name)?;

    let mut sum: i32 = 0;
    let mut cnts: Vec<i32> = vec![1; lines.len()];
    for (idx, curr_line) in lines.iter().enumerate() {
      println!("{}", curr_line);
      let card: Vec<&str> = curr_line.split(": ").collect();
      let cdata: Vec<&str> = card[1].split(" | ").collect();
      let wns: Vec<i32> = cdata[0].split_whitespace().map(|s: &str| s.parse::<i32>().ok().unwrap()).collect();
      let gns: Vec<i32> = cdata[1].split_whitespace().map(|s: &str| s.parse::<i32>().ok().unwrap()).collect();
      let wgns: Vec<i32> = gns.into_iter().filter(|gn| wns.contains(&gn)).collect();
      if wgns.len() > 0 {
        for n in idx + 1..=idx + wgns.len() {
          cnts[n] += cnts[idx];
        }
      }
      sum += cnts[idx];
    }

    Ok(sum)
}
