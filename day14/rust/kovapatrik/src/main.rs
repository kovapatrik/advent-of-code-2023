use std::{collections::HashSet, hash::Hasher, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
  field: Vec<Vec<char>>
}

impl Display for Platform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in &self.field {
      for c in row {
        write!(f, "{}", c)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

#[derive(Debug, Eq)]
struct Cache {
  step: usize,
  platform: Platform
}

impl Cache {
  fn new(step: usize, platform: Platform) -> Cache {
    Cache {
      step,
      platform
    }
  }
}

impl PartialEq for Cache {
  fn eq(&self, other: &Cache) -> bool {
    self.platform == other.platform
  }
}

impl std::hash::Hash for Cache {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.platform.hash(state);
  }
}

impl Platform {

  fn parse(input: &String) -> Platform {
    Platform {
      field: input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
    }
  }

  fn tilt_north(&mut self, x: usize, y: usize) {

    let mut final_i = y;
    for i in (0..y).rev() {
      final_i = i;
      if self.field[i][x] == 'O' || self.field[i][x] == '#' {
        final_i += 1;
        break;
      }
    }
    self.field[y][x] = '.';
    self.field[final_i][x] = 'O';
  }

  fn tilt_west(&mut self, x: usize, y: usize) {

    let mut final_j = x;
    for j in (0..x).rev() {
      final_j = j;
      if self.field[y][j] == 'O' || self.field[y][j] == '#' {
        final_j += 1;
        break;
      }
    }
    self.field[y][x] = '.';
    self.field[y][final_j] = 'O';
  }

  fn tilt_south(&mut self, x: usize, y: usize) {

    let mut final_i = y;
    for i in y+1..self.field.len() {
      final_i = i;
      if self.field[i][x] == 'O' || self.field[i][x] == '#' {
        final_i -= 1;
        break;
      }
    }
    self.field[y][x] = '.';
    self.field[final_i][x] = 'O';
  }

  fn tilt_east(&mut self, x: usize, y: usize) {

    let mut final_j = x;
    for j in x+1..self.field[y].len() {
      final_j = j;
      if self.field[y][j] == 'O' || self.field[y][j] == '#' {
        final_j -= 1;
        break;
      }
    }
    self.field[y][x] = '.';
    self.field[y][final_j] = 'O';
  }

  fn tilt_all(&mut self) {

    for fun in [Self::tilt_north, Self::tilt_west] {
      for i in 0..self.field.len() {
        for j in 0..self.field[i].len() {
          if self.field[i][j] == 'O' {
            fun(self, j, i);
          }
        }
      }
    }

    for fun in [Self::tilt_south, Self::tilt_east] {
      for i in (0..self.field.len()).rev() {
        for j in (0..self.field[i].len()).rev() {
          if self.field[i][j] == 'O' {
            fun(self, j, i);
          }
        }
      }
    }
  }


  fn total_load(&self) -> usize {
    let length = self.field.len();
    self.field
      .iter()
      .enumerate()
      .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (length - i))
      .sum()
  }

  fn part1(&mut self) -> usize {
    for i in 0..self.field.len() {
      for j in 0..self.field[i].len() {
        if self.field[i][j] == 'O' {
          self.tilt_north(j, i);
        }
      }
    }
    self.total_load()
  }

  fn part2(&mut self) -> usize {
      
    let mut cache = HashSet::new();

    for i in 0..1_000_000_000 {

      cache.insert(Cache::new(i, self.clone()));

      self.tilt_all();

      // There is a cycle
      if let Some(elem) = cache.get(&Cache::new(0, self.to_owned())) {
        let cycle_length = i + 1 - elem.step;
        let steps_remaining = 1_000_000_000 - 1 - i;
        let steps_to_skip = steps_remaining % cycle_length;

        for _ in 0..steps_to_skip {
          self.tilt_all();
        }

        return self.total_load();
      }
    }

    0
  }
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut platform = Platform::parse(&input);
  
  println!("Part 1: {}", platform.part1());
  println!("Part 2: {}", platform.part2());
}
