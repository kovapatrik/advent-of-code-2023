fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let first = input
    .lines()
    .map(|line| {
      let num_string = line.as_bytes();
      let first = num_string[line.find(char::is_numeric).unwrap()] - b'0';
      let last = num_string[line.rfind(char::is_numeric).unwrap()] - b'0';
      (first*10+last) as u16
    })
    .sum::<u16>();

  println!("First: {}", first);
  
  let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let second = input
    .lines()
    .map(|line| {
      let num_string = line.as_bytes();
      
      let mut first_idx = match line.find(char::is_numeric) {
        Some(idx) => idx,
        None => line.len()
      };
      let mut last_idx = match line.rfind(char::is_numeric) {
        Some(idx) => idx,
        None => 0
      };
      
      let mut first = num_string[first_idx] - b'0';
      let mut last = num_string[last_idx] - b'0';
      
      for (i, number) in numbers.iter().enumerate() {

        match line.find(number) {
          Some(idx) => {
            if idx < first_idx {
              first_idx = idx;
              first = (i+1) as u8;
            }
          },
          None => ()
        }

        match line.rfind(number) {
          Some(idx) => {
            if idx > last_idx {
              last_idx = idx;
              last = (i+1) as u8;
            }
          },
          None => ()
        }
      }
      (first*10+last) as u16
    })
    .sum::<u16>();

  println!("Second: {}", second);
}