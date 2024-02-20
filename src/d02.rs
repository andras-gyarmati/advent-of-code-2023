use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn calc_p1(file_name: &str) -> Result<i32, io::Error> {
  let cube_count: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
  let file = File::open(file_name)?;
  let reader = BufReader::new(file);
  let mut sum: i32 = 0;

  for line in reader.lines() {
    let line = line?;
    let game: Vec<&str> = line.split(": ").collect();
    let name_str = *game.first().unwrap();
    let id = name_str.split(" ").last().unwrap_or("0").parse::<i32>().unwrap_or(0);
    let mut is_wrong_line = false;
    let values_str = *game.last().unwrap();
    let values: Vec<&str> = values_str.split("; ").collect();
    values.iter().for_each(|&x| {
      let cubes: Vec<&str> = x.split(", ").collect();
      cubes.iter().for_each(|&roll| {
        let split_roll: Vec<&str> = roll.split(" ").collect();
        let num = split_roll.first().unwrap().parse::<i32>().unwrap_or(0);
        let c_name = *split_roll.last().unwrap();
        let limit = *cube_count.get(c_name).unwrap_or(&0);
        if num > limit {
          is_wrong_line = true;
        }
      })
    });
    if !is_wrong_line {
      sum += id;
    }
  }

  Ok(sum)
}

pub fn calc_p2(file_name: &str) -> Result<i32, io::Error> {
  let unique_colors: Vec<&str> = vec!["red", "green", "blue"];
  let file = File::open(file_name)?;
  let reader = BufReader::new(file);
  let mut sum: i32 = 0;

  for line in reader.lines() {
    let line = line?;
    let game_split: Vec<&str> = line.split(": ").collect();
    let flattened_rolls: String = game_split.last().unwrap().replace("; ", ", ");
    let parsed_rolls: Vec<(i32, &str)> = flattened_rolls.split(", ").map(|roll| {
      let split_roll: Vec<&str> = roll.split_whitespace().collect();
      let num: i32 = split_roll.first().unwrap().parse::<i32>().unwrap_or_default();
      let color: &str = *split_roll.last().unwrap();
      (num, color)
    }).collect();
    let max_by_color: Vec<(i32, &str)> = unique_colors.iter().map(|&color| {
      let max: i32 = parsed_rolls.iter().filter(|x| x.1.eq(color)).map(|x| x.0).max().unwrap_or_default();
      (max, color)
    }).collect();
    let colors_multiplied: i32 = max_by_color.iter().map(|x| x.0).product();
    sum = sum + colors_multiplied;
  }

  Ok(sum)
}
