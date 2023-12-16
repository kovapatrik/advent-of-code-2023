use std::collections::HashMap;

#[derive(Debug, Eq, Clone, Copy)]
struct LabelFocal<'a>(&'a str, usize);

impl PartialEq for LabelFocal<'_> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl std::hash::Hash for LabelFocal<'_> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

type FocalMap<'a> = HashMap<u64, Vec<LabelFocal<'a>>>;

struct Sequence<'a> {
  steps: Vec<&'a str>
}

impl<'a> Sequence<'a> {
  fn parse(input: &str) -> Sequence {
    Sequence {
      steps: input
        .trim()
        .split(",")
        .collect()
    }
  }

  fn hash(&self, value: &str) -> u64 {
    let mut h = 0;
    for c in value.chars() {
      h += c as u64;
      h *= 17;
      h %= 256;
    }
    h
  }

  fn part2(&self) -> u64 {
    
    let mut hashmap = FocalMap::new();

    for step in &self.steps {
      
      let operation = step.chars().find(|c| *c == '-' || *c == '=').unwrap();
      match operation {
        '-' => {
          let label = &step[0..step.len()-1];

          let hash = self.hash(label);

          if let Some(l) = hashmap.get_mut(&hash) {
            let new_l = l.iter().cloned().filter(|l| l.0 != label).collect();
            *l = new_l;
          }
        },
        '=' => {
          let focal_length = step.chars().last().unwrap().to_digit(10).unwrap() as usize;
          let label = &step[0..step.len()-2];

          let hash = self.hash(label);

          let set = hashmap.entry(hash).or_insert(Vec::new());

          match set.iter_mut().find(|l| l.0 == label) {
            Some(mut l) => {
              l.1 = focal_length;
            },
            None => {
              set.push(LabelFocal(label, focal_length));
            }
          }
        },
        _ => panic!("Unknown operation: {}", operation)
      }

    }

    hashmap
      .iter()
      .map(|(k,v)| {
        v.iter()
          .enumerate()
          .map(|(i, l)| {
            (i+1) * l.1
          })
          .sum::<usize>() as u64 * (k+1)
      })
      .sum()
  }

  fn part1(&self) -> u64 {
    let mut h = 0;
    for step in &self.steps {
      h += self.hash(step);
    }
    h
  }
}

fn main() {
    
  let input = std::fs::read_to_string("input.txt").unwrap();

  let seq = Sequence::parse(&input);

  println!("Part 1: {}", seq.part1());
  println!("Part 2: {}", seq.part2());
}
