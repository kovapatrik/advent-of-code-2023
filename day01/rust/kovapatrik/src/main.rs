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
      let mut line_str = line.clone().to_string();
      numbers.iter().enumerate().for_each(|(i, number)| {
        line_str = line_str.replace(number, format!("{}{}{}", number, i+1, number).as_str());
      });
      let num_string = line_str.as_bytes();
      let first = num_string[line_str.find(char::is_numeric).unwrap()] - b'0';
      let last = num_string[line_str.rfind(char::is_numeric).unwrap()] - b'0';
      (first*10+last) as u16
    })
    .sum::<u16>();

  println!("Second: {}", second);
}