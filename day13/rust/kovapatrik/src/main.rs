#[derive(Debug, Clone)]
struct Pattern {
  rocks: Vec<Vec<char>>,
}

impl Pattern {

  fn search_reflection(&self, is_part1: bool) -> usize {
    'rows: for i in 0..self.rocks.len()-1 {
      let mut diff = self.rows_diff(i, i+1, is_part1);
      if (is_part1 && diff == 0) || (!is_part1 && diff <= 1) {

        // Check which edge is closer
        let min_distance = std::cmp::min(i, self.rocks.len()-1-i-1);

        for j in 1..=min_distance {
          diff += self.rows_diff(i-j, i+1+j, is_part1);

          if (is_part1 && diff != 0) || (!is_part1 && diff > 1) {
            continue 'rows;
          }
        }

        // In part 2 every pattern has exactly one error
        if !is_part1 && diff == 0 {
          continue 'rows;
        }

        return (i+1) * 100;
      }
    }

    'columns: for i in 0..self.rocks[0].len()-1 {
      let mut diff = self.columns_diff(i, i+1, is_part1);
      if (is_part1 && diff == 0) || (!is_part1 && diff <= 1) {

        // Check which edge is closer
        let min_distance = std::cmp::min(i, self.rocks[0].len()-1-i-1);

        for j in 1..=min_distance {
          diff += self.columns_diff(i-j, i+1+j, is_part1);
          if (is_part1 && diff != 0) || (!is_part1 && diff > 1) {
            continue 'columns;
          }
        }

        // In part 2 every pattern has exactly one error
        if !is_part1 && diff == 0 {
          continue 'columns;
        }

        return i+1;
      }
    }

    return 0;
  }

  fn rows_diff(&self, y1: usize, y2: usize, is_part1: bool) -> usize {
    let iter = self.rocks[y1].iter().zip(self.rocks[y2].iter());
    let mut count = 0;
    for (x1, x2) in iter {
      if x1 != x2 {
        if is_part1 {
          return 1;
        }
        count += 1;
        if count > 1 {
          return count;
        }
      }
    }
    return count;
  }

  fn columns_diff(&self, x1: usize, x2: usize, is_part1: bool) -> usize {
    let mut count = 0;
    for line in self.rocks.iter() {
      if line[x1] != line[x2] {
        if is_part1 {
          return 1;
        }
        count += 1;
        if count > 1 {
          return count;
        }
      }
    }
    return count;
  }

}

fn get_patterns(input: &String) -> Vec<Pattern> {
  input
    .split("\n\n")
    .map(| pattern| {
      let rocks = pattern
        .split("\n")
        .map(|line| {
          line
            .chars()
            .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
      Pattern { rocks }
    })
    .collect::<Vec<Pattern>>()
}

fn part1(patterns: &Vec<Pattern>) -> usize {
  patterns
    .iter()
    .map(|pattern| pattern.search_reflection(true))
    .sum()
}

fn part2(patterns: &Vec<Pattern>) -> usize {
  patterns
    .iter()
    .map(|pattern| pattern.search_reflection(false))
    .sum()
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();

  let patterns = get_patterns(&input);

  println!("Part 1: {}", part1(&patterns));
  println!("Part 2: {}", part2(&patterns));
}
