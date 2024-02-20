macro_rules! parameterized_tests {
  ($(($name:ident, $func:expr, $input_file:expr, $expected:expr),)*) => {
    $(
      #[test]
      fn $name() {
        let result = $func($input_file).unwrap();
        assert_eq!(result, $expected);
      }
    )*
  };
}

#[cfg(test)]
mod tests {
  parameterized_tests! {
    (test_d01_p1, crate::d01::calc_p1, "inputs/d1/test.txt", 142),
    (input_d01_p1, crate::d01::calc_p1, "inputs/d1/input.txt", 54940),
    (test_d01_p2, crate::d01::calc_p2, "inputs/d1/test2.txt", 281),
    (input_d01_p2, crate::d01::calc_p2, "inputs/d1/input.txt", 54208),
    (test_d02_p1, crate::d02::calc_p1, "inputs/d2/test.txt", 8),
    (input_d02_p1, crate::d02::calc_p1, "inputs/d2/input.txt", 2545),
    (test_d02_p2, crate::d02::calc_p2, "inputs/d2/test.txt", 2286),
    (input_d02_p2, crate::d02::calc_p2, "inputs/d2/input.txt", 78111),
  }
}