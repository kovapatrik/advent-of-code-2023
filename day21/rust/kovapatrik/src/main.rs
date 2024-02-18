use std::collections::{HashSet, VecDeque};


struct Map {
  start_position: (usize, usize),
  map: Vec<Vec<char>>,
}

impl Map {
  fn parse(input: &str) -> Map {
    
    let mut map = Vec::new();
    let mut start_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
      let mut row = Vec::new();
      for (x, c) in line.chars().enumerate() {
        row.push(c);
        if c == 'S' {
          start_position = (x, y);
        }
      }
      map.push(row);
    };

    Map { 
      start_position,
      map 
    }
  }

  fn find_garden_plots(&self, max_step: usize) -> usize {

    let max_step_mod = max_step % 2;

    let mut queue = VecDeque::new();
    let mut max_step_nodes = HashSet::new();
    queue.push_back((self.start_position, 0));

    while let Some((position, step)) = queue.pop_front() {
      if max_step_nodes.contains(&position) || step > max_step {
        continue;
      }

      if step % 2 == max_step_mod && step <= max_step {
        max_step_nodes.insert(position);
      }

      let (x, y) = position;

      let diffs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

      for (dx, dy) in diffs {
        let (new_x, new_y) = (x as isize + dx, y as isize + dy);
        if new_x < 0 || new_y < 0 || new_x >= self.map[0].len() as isize || new_y >= self.map.len() as isize {
          continue;
        }
        let (new_x, new_y) = (new_x as usize, new_y as usize);
        if self.map[new_y][new_x] == '#' {
          continue;
        }
        queue.push_back(((new_x, new_y), step + 1));
      }
    }

    max_step_nodes.len()
  }

  fn part1(&self) -> usize {
    self.find_garden_plots(64)
  }

  fn part2(&self) -> usize {
    self.find_garden_plots(50)
  }
}

fn main() {
  
  let input = std::fs::read_to_string("sample.txt").unwrap();

  let map = Map::parse(&input);

  // println!("Part 1: {}", map.part1());
  println!("Part 2: {}", map.part2());
}
