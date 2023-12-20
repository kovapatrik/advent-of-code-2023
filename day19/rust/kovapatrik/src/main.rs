use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Clone, Copy)]
enum AcceptReject {
  Accept,
  Reject
}

#[derive(Debug, Clone)]
enum Destination {
  Decision(AcceptReject),
  NextWorkflow(String)
}

#[derive(Debug, Clone, Copy)]
enum Category {
  ExtremelyCoolLooking,
  Musical,
  Aerodynamic,
  Shiny
}

type Part = [usize; 4];

#[derive(Debug, Clone)]
enum Rule {
  Conditional(Category, Ordering, usize, Destination),
  Terminal(Destination)
}

struct System {
  parts: Vec<Part>,
  rules: HashMap<String, Vec<Rule>>
}

type RangePart = [(usize, usize); 4];

impl System {
  fn parse(input: &String) -> System {

    let mut rules_map = HashMap::new();

    let (rules, parts) = input.split_once("\n\n").unwrap();

    rules.split("\n").filter(|r| !r.is_empty()).for_each(|r| {

      let (workflow_name, rule) = r.split_once("{").unwrap();
      let rule = &rule[..rule.len()-1];
      
      let current_rules =rule.split(",").map(|sr| {
        let operation_idx = sr.find(|c: char| c == '<' || c == '>').unwrap_or(0);

        if operation_idx == 0 {
          match sr {
            "A" => return Rule::Terminal(Destination::Decision(AcceptReject::Accept)),
            "R" => return Rule::Terminal(Destination::Decision(AcceptReject::Reject)),
            _ => return Rule::Terminal(Destination::NextWorkflow(sr.to_string()))
          }
        }

        let category = match &sr[..operation_idx] {
          "x" => Category::ExtremelyCoolLooking,
          "m" => Category::Musical,
          "a" => Category::Aerodynamic,
          "s" => Category::Shiny,
          _ => panic!("Unknown category")
        };

        let ordering = match &sr[operation_idx..operation_idx+1] {
          "<" => Ordering::Less,
          ">" => Ordering::Greater,
          _ => panic!("Unknown ordering")
        };

        let value = &sr[operation_idx+1..];
        let (value, destination) = value.split_once(":").unwrap();

        let value = value.parse::<usize>().unwrap();

        let destination = match destination {
          "A" => Destination::Decision(AcceptReject::Accept),
          "R" => Destination::Decision(AcceptReject::Reject),
          _ => Destination::NextWorkflow(destination.to_string())
        };

        Rule::Conditional(category, ordering, value, destination)
      }).collect::<Vec<Rule>>();

      rules_map.insert(workflow_name.to_string(), current_rules);
    });

    let parts = parts
      .split("\n")
      .filter(|p| !p.is_empty())
      .map(|p| {
        p[1..p.len()-1]
          .split(",")
          .map(|v| v[2..].parse::<usize>().unwrap())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap()
      }).collect();

    System {
      parts,
      rules: rules_map
    }
  }

  fn apply_worklow(&self, part: &Part, start: String) -> usize {

    let mut start = Destination::NextWorkflow(start);

    loop {

      // println!("{:?} {:?}", part, start);

      match start {
        Destination::Decision(AcceptReject::Accept) => {
          return part.iter().sum();
        },
        Destination::Decision(AcceptReject::Reject) => {
          return 0;
        },
        Destination::NextWorkflow(ref workflow) => {
          let rules = self.rules.get(workflow).unwrap();
          for r in rules {
            match r {
              Rule::Conditional(category, ordering, value, dest) => {
                let part_value = part[*category as usize];
                let cond = match ordering {
                  Ordering::Less => part_value < *value,
                  Ordering::Greater => part_value > *value,
                  _ => panic!("Unknown ordering")
                };
                if cond {
                  start = dest.clone();
                  break;
                }
              },
              Rule::Terminal(dest) => {
                start = dest.clone();
                break;
              }
            }
          };
        }
      }
    }
  }

  fn apply_worklow_p2(&self, start: String) -> usize {

    let ranges: RangePart = [(1, 4000); 4];
    let mut combinations = 0;

    let mut nodes = vec![(ranges, Destination::NextWorkflow(start))];

    while let Some((mut ranges, start)) = nodes.pop() {
      match start {
        Destination::Decision(AcceptReject::Accept) => {
          combinations += ranges.iter().map(|(min, max)| (*min..=*max).count()).product::<usize>();
          continue;
        },
        Destination::Decision(AcceptReject::Reject) => {
          continue;
        },
        Destination::NextWorkflow(ref workflow) => {
          let rules = self.rules.get(workflow).unwrap();
          for r in rules {
            match r {
              Rule::Conditional(category, ordering, value, dest) => {
                let mut inside_range = ranges;
                let (min, max) = inside_range[*category as usize];
                inside_range[*category as usize] = match ordering {
                  Ordering::Less => (min, usize::min(max, value - 1)),
                  Ordering::Greater => (usize::max(min, value + 1), max),
                  _ => panic!("Unknown ordering")
                };

                nodes.push((inside_range, dest.clone()));

                let (min, max) = ranges[*category as usize];

                ranges[*category as usize] = match ordering {
                  Ordering::Less => (usize::max(min, *value), max),
                  Ordering::Greater => (min, usize::min(max, *value)),
                  _ => panic!("Unknown ordering")
                };

                if ranges.iter().map(|(min, max)| (*min..=*max).count()).product::<usize>() == 0 {
                  break;
                }
              },
              Rule::Terminal(dest) => {
                nodes.push((ranges, dest.clone()));
              }
            }
          };
        }
      }
    }

    combinations
  }

  fn part1(&self) -> usize {
    self.parts.iter()
      .map(|p| self.apply_worklow(p, "in".to_string()))
      .sum()
  }

  fn part2(&self) -> usize {
    self.apply_worklow_p2("in".to_string())
  }
}


fn main() {
    
  let input = std::fs::read_to_string("input.txt").unwrap();

  let system = System::parse(&input);

  println!("Part 1: {}", system.part1());
  println!("Part 2: {}", system.part2());
}
