use std::collections::VecDeque;

struct Sequence {
  values: Vec<i32>,
}

fn get_sequences(input: &String) -> Vec<Sequence> {
  input
    .lines()
    .map(|line| Sequence {
      values: line
        .split(' ')
        .map(|value| value.parse::<i32>().unwrap())
        .collect(),
    })
    .collect()
}

fn part1(sequences: &Vec<Sequence>) -> i32 {
 
  sequences
    .iter()
    .map(|seq| {
      let mut vec = seq.values.clone();
      let mut last_elems = Vec::new();
      while !vec.iter().all(|v| *v == 0) {
        let elem = vec.iter().last().unwrap().clone();
        last_elems.push(elem);
        vec = vec.windows(2).map(|w| w[1] - w[0]).collect();
      }
      last_elems.iter().fold(0, |acc, f| acc + f)
    })
    .sum()
}

fn part2(sequences: &Vec<Sequence>) -> i32 {
  
  sequences
    .iter()
    .map(|seq| {
      let mut vec = seq.values.clone();
      let mut first_elems = VecDeque::new();
      while !vec.iter().all(|v| *v == 0) {
        let elem = vec[0];
        first_elems.push_front(elem);
        vec = vec.windows(2).map(|w| w[1] - w[0]).collect();
      }
      first_elems.iter().fold(0, |acc, f| f - acc)
    })
    .sum()
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();
  let sequences = get_sequences(&input);

  println!("Part 1: {}", part1(&sequences));
  println!("Part 2: {}", part2(&sequences));
}
