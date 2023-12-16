use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
  x: usize,
  y: usize,
  direction: Direction,
}

impl Beam {
  fn new(x: usize, y: usize, direction: Direction) -> Self {
    Self { x, y, direction}
  }

  fn step(&mut self, grid: &Grid, seen: &mut HashSet<Beam>) -> bool {

    if seen.contains(self) {
      return false;
    }

    seen.insert(self.clone());

    match self.direction {
      Direction::Up => {
        if self.y == 0 {
          return false;
        }
        self.y -= 1;
      },
      Direction::Down => {
        if self.y ==  grid.grid.len() - 1 {
          return false;
        }
        self.y += 1;
      }
      Direction::Left => {
        if self.x == 0 {
          return false;
        }
        self.x -= 1;
      }
      Direction::Right => {
        if self.x == grid.grid[0].len() - 1 {
          return false;
        }
        self.x += 1;
      }
    }
    return true;
  }
}

#[derive(Debug)]
struct Grid {
  grid: Vec<Vec<char>>,
}

impl Grid {
  fn parse(input: &str) -> Self {
    let grid = input
      .lines()
      .map(|line| line.chars().collect())
      .collect::<Vec<Vec<char>>>();
    Self { grid }
  }

  fn beam_process(&self, seen: &mut HashSet<Beam>, start_beam: Beam) -> usize {

    let mut to_process = VecDeque::from([start_beam.clone()]);

    while !to_process.is_empty() {

      let mut beam = to_process.pop_front().unwrap();

      if !beam.step(&self, seen) {
        continue;
      }

      match self.grid[beam.y][beam.x] {
        '/' => {
          beam.direction = match beam.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
          };
          to_process.push_back(beam);
        },
        '\\' => {
          beam.direction = match beam.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
          };
          to_process.push_back(beam);
        },
        '|' => match beam.direction {
          Direction::Down | Direction::Up => to_process.push_back(beam),
          Direction::Left | Direction::Right => {
            let mut u_beam = beam.clone();
            u_beam.direction = Direction::Up;
            to_process.push_back(u_beam);

            let mut d_beam = beam.clone();
            d_beam.direction = Direction::Down;
            to_process.push_back(d_beam);
          },
        },
        '-' => match beam.direction {
          Direction::Down | Direction::Up => {
            let mut l_beam = beam.clone();
            l_beam.direction = Direction::Left;
            to_process.push_back(l_beam);

            let mut r_beam = beam.clone();
            r_beam.direction = Direction::Right;
            to_process.push_back(r_beam);
          },
          Direction::Left | Direction::Right => to_process.push_back(beam),
        },
        _ => to_process.push_back(beam),
      }
    }

    seen.iter()
      .map(|beam| (beam.x, beam.y))
      .collect::<HashSet<(usize, usize)>>()
      .len()
  }

  fn part1(&self) -> usize {

    let mut seen: HashSet<Beam> = HashSet::new();

    let start_direction = match self.grid[0][0] {
      '\\' => Direction::Down,
      '-' | '.' => Direction::Right,
      '|' => Direction::Down,
      _ => panic!("Invalid start direction"),
    };

    let start_beam = Beam::new(0, 0, start_direction);


    self.beam_process(&mut seen, start_beam)
  }

  fn part2(&self) -> usize {

    let mut max = 0;
    
    let binding = [0, self.grid.len() - 1];
    let row_iter = binding.iter();
    let row_col_iter = 0..self.grid[0].len();

    for &y in row_iter {
      for x in row_col_iter.clone() {
        let mut seen: HashSet<Beam> = HashSet::new();

        let beam_direction = match y {
          0 => Direction::Down,
          _ => Direction::Up,
        };

        let start_beam = Beam::new(x, y, beam_direction);
        let result = self.beam_process(&mut seen, start_beam);
        if result > max {
          max = result;
        }
      }
    }

    let binding = [0, self.grid[0].len() - 1];
    let col_iter = binding.iter();
    let col_row_iter = 0..self.grid.len();

    for &x in col_iter {
      for y in col_row_iter.clone() {
        let mut seen: HashSet<Beam> = HashSet::new();

        let beam_direction = match x {
          0 => Direction::Right,
          _ => Direction::Left,
        };

        let start_beam = Beam::new(x, y, beam_direction);
        let result = self.beam_process(&mut seen, start_beam);
        if result > max {
          max = result;
        }
      }
    }

    max - 1
  }

}

fn main() {
    
  let input = std::fs::read_to_string("input.txt").unwrap();

  let grid = Grid::parse(&input);

  println!("Part 1: {}", grid.part1());
  println!("Part 2: {}", grid.part2());
}
