#[derive(Clone, Debug, Copy)]
struct Pipe {
  symbol: char,
  x: u32,
  y: u32,
}

impl std::fmt::Display for Pipe {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}({},{})", self.symbol, self.x, self.y)
  }
}

impl Pipe {
  fn follow(self, prev_pipe: &Pipe) -> Option<(u32, u32)> {
    match self.symbol {
      '|' => {
        if prev_pipe.y != self.y {
          return None
        }
        if prev_pipe.x == self.x + 1 {
          Some((self.x - 1, self.y))
        } else if prev_pipe.x == self.x - 1 {
          Some((self.x + 1, self.y))
        } else {
          None
        }
      },
      '-' => {
        if prev_pipe.x != self.x {
          return None
        }
        if prev_pipe.y == self.y + 1 {
          Some((self.x, self.y - 1))
        } else if prev_pipe.y == self.y - 1 {
          Some((self.x, self.y + 1))
        } else {
          None
        }
      },
      'L' => {
        if prev_pipe.x == self.x && prev_pipe.y == self.y + 1 {
          Some((self.x - 1, self.y))
        } else if prev_pipe.y == self.y && prev_pipe.x == self.x - 1 {
          Some((self.x, self.y + 1))
        } else {
          None
        }
      },
      'J' => {
        if prev_pipe.x == self.x && prev_pipe.y == self.y - 1 {
          Some((self.x - 1, self.y))
        } else if prev_pipe.y == self.y && prev_pipe.x == self.x - 1 {
          Some((self.x, self.y - 1))
        } else {
          None
        }
      },
      '7' => {
        if prev_pipe.x == self.x && prev_pipe.y == self.y - 1 {
          Some((self.x + 1, self.y))
        } else if prev_pipe.y == self.y && prev_pipe.x == self.x + 1 {
          Some((self.x, self.y - 1))
        } else {
          None
        }
      },
      'F' => {
        if prev_pipe.x == self.x && prev_pipe.y == self.y + 1 {
          Some((self.x + 1, self.y))
        } else if prev_pipe.y == self.y && prev_pipe.x == self.x + 1 {
          Some((self.x, self.y + 1))
        } else {
          None
        }
      },
      _ => panic!("Unknown pipe symbol: {}", self.symbol),
    }
  }
}

fn get_pipes(input: &String) -> Vec<Pipe> {
  input
    .lines()
    .enumerate()
    .flat_map(|(x, line)| {
      line
        .chars()
        .enumerate()
        .filter_map(|(y, symbol)| {
          if symbol == '.' {
            None
          } else {
            Some(Pipe { symbol, x: x as u32, y: y as u32 })
          }
        })
        .collect::<Vec<Pipe>>()
    })
    .collect()
}

fn part1(pipes: &Vec<Pipe>) -> (u32, Vec<Pipe>) {
  let mut touched_pipes = Vec::new();
  let start_pipe = pipes.iter().find(|p| p.symbol == 'S').unwrap();
  let mut prev_pipe = pipes.iter().find(|p| p.symbol != 'S' && p.follow(start_pipe).is_some()).unwrap();
  let mut next_pipe_coords = prev_pipe.follow(start_pipe).unwrap();
  let mut next_pipe = pipes.iter().find(|p| p.x == next_pipe_coords.0 && p.y == next_pipe_coords.1).unwrap();
  let mut count = 1;
  touched_pipes.push(start_pipe.clone());
  touched_pipes.push(prev_pipe.clone());
  while next_pipe.symbol != 'S' {
    touched_pipes.push(next_pipe.clone());
    next_pipe_coords = next_pipe.follow(prev_pipe).unwrap();
    prev_pipe = next_pipe;
    next_pipe = pipes.iter().find(|p| p.x == next_pipe_coords.0 && p.y == next_pipe_coords.1).unwrap();
    count += 1;
  }
  touched_pipes.push(start_pipe.clone());
  (((count as f32) / 2.0).ceil() as u32, touched_pipes)
}

fn part2(touched_pipes: &Vec<Pipe>) -> u32 {

  let area = touched_pipes
    .windows(2)
    .map(|w| {
      let p0 = w[0];
      let p1 = w[1];
      ((p0.x*p1.y) as i32) - ((p0.y*p1.x) as i32)
    })
    .sum::<i32>()
    .abs() as u32;
  (area / 2) - ((touched_pipes.len()-1)/2) as u32 + 1
}

fn main() {

  let input = std::fs::read_to_string("input.txt").unwrap();
  let pipes = get_pipes(&input);

  // println!("{pipes:?}");
  let (part1_result, touched_pipes) = part1(&pipes);

  println!("Part 1: {}", part1_result);
  println!("Part 2: {}", part2(&touched_pipes));
}
