#[test]
fn test_d01_p1() {
  assert_eq!(crate::d01::calc_p1("inputs/d1/test.txt").unwrap(), 142);
}

#[test]
fn input_d01_p1() {
  assert_eq!(crate::d01::calc_p1("inputs/d1/input.txt").unwrap(), 54940);
}

#[test]
fn test_d01_p2() {
  assert_eq!(crate::d01::calc_p2("inputs/d1/test2.txt").unwrap(), 281);
}

#[test]
fn input_d01_p2() {
  assert_eq!(crate::d01::calc_p2("inputs/d1/input.txt").unwrap(), 54208);
}

#[test]
fn test_d02_p1() {
  assert_eq!(crate::d02::calc_p1("inputs/d2/test.txt").unwrap(), 8);
}

#[test]
fn input_d02_p1() {
  assert_eq!(crate::d02::calc_p1("inputs/d2/input.txt").unwrap(), 2545);
}

#[test]
fn test_d02_p2() {
  assert_eq!(crate::d02::calc_p2("inputs/d2/test.txt").unwrap(), 2286);
}

#[test]
fn input_d02_p2() {
  assert_eq!(crate::d02::calc_p2("inputs/d2/input.txt").unwrap(), 78111);
}

#[test]
fn test_d03_p1() {
  assert_eq!(crate::d03::calc_p1("inputs/d3/test.txt").unwrap(), 4361);
}

#[test]
fn input_d03_p1() {
  assert_eq!(crate::d03::calc_p1("inputs/d3/input.txt").unwrap(), 539590);
}

#[test]
fn test_d03_p2() {
  assert_eq!(crate::d03::calc_p2("inputs/d3/test.txt").unwrap(), 467835);
}

#[test]
fn input_d03_p2() {
  assert_eq!(crate::d03::calc_p2("inputs/d3/input.txt").unwrap(), 80703636);
}

#[test]
fn test_d04_p1() {
  assert_eq!(crate::d04::calc_p1("inputs/d4/test.txt").unwrap(), 13);
}

#[test]
fn input_d04_p1() {
  assert_eq!(crate::d04::calc_p1("inputs/d4/input.txt").unwrap(), 20855);
}

#[test]
fn test_d04_p2() {
  assert_eq!(crate::d04::calc_p2("inputs/d4/test.txt").unwrap(), 30);
}

#[test]
fn input_d04_p2() {
  assert_eq!(crate::d04::calc_p2("inputs/d4/input.txt").unwrap(), 5489600);
}

#[test]
fn test_d05_p1() {
  assert_eq!(crate::d05::calc_p1("inputs/d5/test.txt").unwrap(), 0);
}

#[test]
fn input_d05_p1() {
  assert_eq!(crate::d05::calc_p1("inputs/d5/input.txt").unwrap(), 0);
}
