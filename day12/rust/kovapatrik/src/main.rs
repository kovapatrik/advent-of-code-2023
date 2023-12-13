use std::{collections::HashMap, cmp};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Spring {
  Operational,
  Damaged,
  Unkown
}

impl From<char> for Spring {
  fn from(c: char) -> Self {
    match c {
      '.' => Spring::Operational,
      '#' => Spring::Damaged,
      _ => Spring::Unkown,
    }
  }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Record {
  springs: Vec<Spring>,
  sizes: Vec<usize>,
}

type Cache = HashMap<Record, usize>;

fn get_records(input: &String) -> Vec<Record> {
  input
    .lines()
    .map(|line| {
      let (springs, sizes) = line.split_once(' ').unwrap();
      let springs = springs
        .chars()
        .map(|s| Spring::from(s))
        .collect::<Vec<Spring>>();
      let sizes = sizes
        .split(',')
        .map(|group| group.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

        Record {
          springs,
          sizes,
        }
    })
    .collect::<Vec<Record>>()
}

fn find_possible_arrangements(cache: &mut HashMap<Record, usize>, record: &Record) -> usize {

  if let Some(&r) = cache.get(record) {
    return r;
  }

  // If there is no groups left and there is a '#' in the rest of the string, then this is not a valid arrangement
  if record.sizes.is_empty() {
    let v = match record.springs.iter().any(|s| *s == Spring::Damaged) {
      true => 0,
      false => 1,
    };
    cache.insert(record.clone(), v);
    return v;
  }

  // If there is not enough springs to fill the groups, then this is not a valid arrangement
  if record.springs.len() < record.sizes.iter().sum::<usize>() + record.sizes.len() - 1 {
    cache.insert(record.clone(), 0);
    return 0;
  }

  if record.springs[0] == Spring::Operational {
    let solutions = find_possible_arrangements(cache, &Record {
      springs: record.springs[1..].to_vec(),
      sizes: record.sizes.clone(),
    });
    cache.insert(record.clone(), solutions);
    return solutions;
  }

  let mut solutions = 0;
  let current_size = record.sizes[0];

  let is_all_non_operational = record.springs[..current_size].iter().all(|s| *s != Spring::Operational);

  if is_all_non_operational && 
      ((record.springs.len() > current_size && record.springs[current_size] != Spring::Damaged) ||
      record.springs.len() <= current_size) {

    let end = cmp::min(record.springs.len(), current_size+1);
    solutions = find_possible_arrangements(cache, &Record {
      springs: record.springs[end..].to_vec(),
      sizes: record.sizes[1..].to_vec(),
    });
  }

  if record.springs[0] == Spring::Unkown {
    solutions += find_possible_arrangements(cache, &Record {
      springs: record.springs[1..].to_vec(),
      sizes: record.sizes.clone(),
    });
  }

  cache.insert(record.clone(), solutions);
  solutions
}

fn part1(cache: &mut Cache, records: &Vec<Record>) -> usize {
  records
    .iter()
    .map(|r| find_possible_arrangements(cache, r))
    .sum()
}

fn part2(cache: &mut Cache, records: &Vec<Record>) -> usize {
  records
    .iter()
    .map(|r| {
      let springs = r.springs
        .iter()
        .cloned()
        .chain([Spring::Unkown].iter().cloned())
        .cycle()
        .take(r.springs.len() * 5 + 4)
        .collect::<Vec<Spring>>();

      let sizes = r.sizes
        .iter()
        .cloned()
        .cycle()
        .take(r.sizes.len() * 5)
        .collect::<Vec<usize>>();

      let r = Record {
        springs,
        sizes,
      };

      find_possible_arrangements(cache, &r)
    })
    .sum()
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut cache = Cache::new();
  let springs = get_records(&input);

  println!("Part 1: {}", part1(&mut cache, &springs));
  println!("Part 2: {}", part2(&mut cache, &springs));
}
