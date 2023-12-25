use std::collections::{HashMap, VecDeque};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Pulse {
  Low,
  High
}

struct Communication {
  source: String,
  pulse: Pulse,
  destination: String
}


trait HandleCommunication {
  fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication>;
}

enum State {
  On,
  Off
}

struct FlipFlop {
  _name: String,
  state: State,
  outputs: Vec<String>
}

impl FlipFlop {
  fn new(name: String, outputs: Vec<String>) -> FlipFlop {
    FlipFlop {
      _name: name,
      state: State::Off,
      outputs
    }
  }
}

impl HandleCommunication for FlipFlop {
  fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {
    match communication.pulse {
      Pulse::Low => {
        match self.state {
          State::On => {
            self.state = State::Off;
            self.outputs
              .iter()
              .map(|o| Communication {
                source: communication.destination.to_owned(),
                pulse: Pulse::Low,
                destination: o.to_owned()
              })
              .collect::<Vec<Communication>>()
          },
          State::Off => {
            self.state = State::On;
            self.outputs
              .iter()
              .map(|o| Communication {
                source: communication.destination.to_owned(),
                pulse: Pulse::High,
                destination: o.to_owned()
              })
              .collect::<Vec<Communication>>()
          }
        }
      },
      Pulse::High => vec![]
    }
  }
}

struct Conjuction {
  name: String,
  inputs: HashMap<String, Pulse>,
  outputs: Vec<String>
}

impl Conjuction {
  fn new(name: String, inputs: Vec<String>, outputs: Vec<String>) -> Conjuction {
    Conjuction {
      name: name,
      inputs: inputs.into_iter().map(|i| (i, Pulse::Low)).collect::<HashMap<String, Pulse>>(),
      outputs
    }
  }
}

impl HandleCommunication for Conjuction {
  fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {

    self.inputs.insert(communication.source.clone(), communication.pulse);

    let pulse_to_send = if self.inputs.values().all(|p| p == &Pulse::High) {
      Pulse::Low
    } else {
      Pulse::High
    };
    
    self.outputs
      .iter()
      .map(|o| Communication {
        source: self.name.to_owned(),
        pulse: pulse_to_send,
        destination: o.to_owned()
      })
      .collect::<Vec<Communication>>()
  }
}

struct Broadcaster {
  name: String,
  outputs: Vec<String>
}

impl Broadcaster {
  fn new(name: String, outputs: Vec<String>) -> Broadcaster {
    Broadcaster {
      name: name,
      outputs
    }
  }
}

impl HandleCommunication for Broadcaster {
  fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {
    self.outputs
      .iter()
      .map(|o| Communication {
        source: self.name.to_owned(),
        pulse: communication.pulse.clone(),
        destination: o.to_owned()
      })
      .collect::<Vec<Communication>>()
  }
}

struct Configuration {
  modules: HashMap<String, Box<dyn HandleCommunication>>
}

impl Configuration {
  fn parse(input: &String) -> Configuration {

    let mut modules: HashMap<String, Box<dyn HandleCommunication>> = HashMap::new();

    let temp_modules = input
      .lines()
      .map(|l| {

        let (source, destinations) = l.split_once(" -> ").unwrap();

        let (mod_type, name) = match source.chars().next().unwrap() {
          'b' => ('b', source.to_owned()),
          '%' => ('%', source[1..].to_owned()),
          '&' => ('&', source[1..].to_owned()),
          _ => panic!("Unknown module")
        };

        let destinations = destinations
          .split(", ")
          .map(|d| d.to_owned())
          .collect::<Vec<String>>();

        (mod_type, name, destinations)
      })
      .collect::<Vec<_>>();

    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (_, name, destinations) in &temp_modules {
      for d in destinations {
        inputs
          .entry(d.to_owned())
          .or_default()
          .push(name.to_owned());
      }
    }

    for (mod_type, name, destinations) in temp_modules {
      let module: Box<dyn HandleCommunication> = match mod_type {
        'b' => Box::new(Broadcaster::new(name.to_owned(), destinations)),
        '&' => Box::new(Conjuction::new(name.to_owned(), inputs[&name.to_owned()].to_owned(), destinations)),
        '%' => Box::new(FlipFlop::new(name.to_owned(), destinations)),
        _ => panic!("Unknown module")
      };
      modules.insert(name, module);
    }

    Configuration {
      modules
    }
  }

  fn push<F>(&mut self, mut helper_fn: F) -> (usize, usize)
    where F: FnMut(&Communication) -> ()
  {

    let mut low_count = 0;
    let mut high_count = 0;

    let mut queue: VecDeque<Communication> = VecDeque::new();
    queue.push_back(Communication {
      source: "button".to_owned(),
      pulse: Pulse::Low,
      destination: "broadcaster".to_owned()
    });

    while let Some(communication) = queue.pop_front() {

      match communication.pulse {
        Pulse::Low => low_count += 1,
        Pulse::High => high_count += 1
      }

      let module = match self.modules.get_mut(&communication.destination) {
        Some(m) => m,
        None => continue
      };

      helper_fn(&communication);

      let new_communications = module.handle_communication(&communication);
      queue.extend(new_communications);
    }

    (low_count, high_count)
  }

  fn part1(&mut self) -> usize {

    let (low_total, high_total) = (0..1000)
      .map(|_| self.push( |_| () ))
      .fold((0,0), |(lt, ht), (l, h)| (lt + l, ht + h));

    low_total * high_total
  }

  fn part2(&mut self) -> usize {

    // lg is the node which is connected to rx
    // lg is conjuction
    // vg, nb, vc, ls are connected to lg, all of them must be high for rx to be low 

    let nodes = ["vg".to_owned(), "nb".to_owned(), "vc".to_owned(), "ls".to_owned()];
    let mut cycle_map = HashMap::<String, Vec<usize>>::new();

    let mut cycle_count = 0;
    while cycle_map.is_empty() || cycle_map.values().any(|v| v.len() < 2) {

      let helper_fn = |communication: &Communication| {
        if nodes.contains(&communication.source) && communication.pulse == Pulse::High {
          cycle_map
            .entry(communication.source.to_owned())
            .or_default()
            .push(cycle_count);
        };
      };

      self.push(helper_fn);
      cycle_count += 1;
    };

    let cycle_lengths = cycle_map
      .values()
      .map(|v| v[1] - v[0])
      .collect::<Vec<_>>();

    
    lcm(&cycle_lengths)
  }
}

pub fn lcm(nums: &[usize]) -> usize {
  if nums.len() == 1 {
      return nums[0];
  }
  let a = nums[0];
  let b = lcm(&nums[1..]);
  a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
  if b == 0 {
      return a;
  }
  gcd_of_two_numbers(b, a % b)
}

fn main() {

  let input = std::fs::read_to_string("input.txt").unwrap();


  let mut configuration = Configuration::parse(&input);
  println!("Part 1: {}", configuration.part1());

  let mut configuration = Configuration::parse(&input);
  println!("Part 2: {}", configuration.part2());
}
