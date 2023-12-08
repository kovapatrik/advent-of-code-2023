use std::collections::{HashMap, VecDeque};

type Network = HashMap<String, (String, String)>;

fn gcd(x: usize, y: usize) -> usize{
    match x == 0{
        true => y,
        false => gcd(y%x, x),
    }
}

fn lcm_vec(nums: Vec<usize>) -> usize {
  nums.iter().fold(1, |acc, x| (acc * x) / gcd(acc, x.clone())) 
}

fn get_network(input: &String) -> Network {

  let mut network = Network::new();

  input
    .lines()
    .skip(2)
    .for_each(|line| {
      let parts = line.split_once(" = ").unwrap();
      let from = parts.0.trim();
      let to_parts = parts.1.split_once(", ").unwrap();

      network.insert(from.to_string(), (to_parts.0[1..].to_string(), to_parts.1[..to_parts.1.len()-1].to_string()));
    });
  
  network
}

fn part1(instuctions: &Vec<char>, network: &Network) -> usize {
  let mut steps: usize = 0;
  let mut current = "AAA";
  let mut instuctions_cpy = VecDeque::from(instuctions.clone());
  while current != "ZZZ" {
    let next = network.get(current).unwrap();

    if instuctions_cpy.is_empty() {
      instuctions_cpy = VecDeque::from(instuctions.clone());
    }

    if instuctions_cpy.pop_front().unwrap() == 'L' {
      current = &next.0;
    } else {
      current = &next.1;
    }
    steps += 1;
  }
  steps
}

fn part2(instuctions: &Vec<char>, network: &Network) -> usize {
  let mut steps: usize = 0;
  let mut instuctions_cpy = VecDeque::from(instuctions.clone());

  let mut starting_nodes = network.keys().filter(|key| key.ends_with("A")).collect::<Vec<&String>>();
  let mut counts = Vec::new();

  while !starting_nodes.is_empty(){

    if instuctions_cpy.is_empty() {
      instuctions_cpy = VecDeque::from(instuctions.clone());
    }

    let current_inst = instuctions_cpy.pop_front().unwrap();

    starting_nodes = starting_nodes
      .iter()
      .filter_map(|node| {
        let next = network.get(*node).unwrap();
        if node.ends_with("Z") {
          counts.push(steps);
          None
        } else {
          match current_inst {
            'L' => Some(&next.0),
            'R' => Some(&next.1),
            _ => None
          }
        }
      })
      .collect::<Vec<&String>>();

    steps += 1;
  }

  lcm_vec(counts)
}
 
fn main() {

  let input = std::fs::read_to_string("input.txt").unwrap();

  let instrucions = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
  let network = get_network(&input);

  println!("Part 1: {}", part1(&instrucions, &network));
  println!("Part 2: {}", part2(&instrucions, &network));
}
