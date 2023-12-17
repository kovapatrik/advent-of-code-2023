use std::collections::{HashMap, BinaryHeap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn opposite(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }

  fn diff(&self) -> (isize, isize) {
    match self {
      Direction::Up => (-1, 0),
      Direction::Down => (1, 0),
      Direction::Left => (0, -1),
      Direction::Right => (0, 1),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
  position: (usize, usize),
  direction: Direction,
  direction_count: usize,
  cost: usize,
}

impl Ord for Node {
  fn cmp(&self, other: &Node) -> std::cmp::Ordering {
    other.cost.cmp(&self.cost)
      .then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

struct Map {
  blocks: Vec<Vec<u32>>
}

#[derive(Eq, Hash, PartialEq)]
struct NodeKey {
  position: (usize, usize),
  direction: Direction,
  direction_count: usize,
}

impl Map {
  fn parse(input: &String) -> Map {
  
    let blocks = input
      .lines()
      .map(|line| {
        line
          .chars()
          .map(|c| c.to_digit(10).unwrap())
          .collect()
      })
      .collect();

    Map { blocks }
  }

  fn find_shortest_path(&self, start: (usize, usize), end: (usize, usize), is_part1: bool) -> Option<usize> {
    // Dijsktra's algorithm

    let mut distances: HashMap<NodeKey, usize> = HashMap::new();

    // min-heap
    let mut queue = BinaryHeap::new();

    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    distances.insert(NodeKey { position: start, direction: Direction::Right, direction_count: 0 }, 0);
    distances.insert(NodeKey { position: start, direction: Direction::Down, direction_count: 0 }, 0);
    
    queue.push(Node { position: start, direction: Direction::Right, direction_count: 0, cost: 0 });
    
    while let Some(Node { cost, position, direction, direction_count }) = queue.pop() {

      if position == end && (is_part1 || direction_count >= 4) {
        return Some(cost);
      }

      let key = NodeKey { position, direction, direction_count };
      if distances.contains_key(&key) && distances[&key] < cost {
        continue;
      }

      for d in directions.iter().filter(|d| **d != direction.opposite()) {
        let (dx, dy) = d.diff();
        
        if position.0 as isize + dy < 0 || position.0 as isize + dy >= self.blocks.len() as isize || position.1 as isize + dx < 0 || position.1 as isize + dx >= self.blocks[0].len() as isize {
          continue;
        }

        let new_position = ((position.0 as isize + dy) as usize, (position.1 as isize + dx) as usize);
        
        let next_node = Node {
          position: new_position,
          direction: *d,
          direction_count: if *d == direction { direction_count + 1 } else { 1 },
          cost: cost + self.blocks[new_position.0][new_position.1] as usize,
        };

        let next_key = NodeKey { 
          position: new_position, 
          direction: *d, 
          direction_count: next_node.direction_count
        };

        if ((is_part1 && next_node.direction_count <= 3) || (!is_part1 && (direction == *d || direction_count >= 4) && next_node.direction_count <= 10)) && (!distances.contains_key(&next_key) || next_node.cost < distances[&next_key]) {
          distances.insert(next_key, next_node.cost);
          queue.push(next_node);
        }
      }
    }
    None
  }
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();

  let map = Map::parse(&input);

  println!("Part 1: {}", map.find_shortest_path((0,0), (map.blocks.len()-1,map.blocks[0].len()-1), true).unwrap());
  println!("Part 2: {}", map.find_shortest_path((0,0), (map.blocks.len()-1,map.blocks[0].len()-1), false).unwrap());
}
