
#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl From<char> for Direction {
  fn from(c: char) -> Self {
    match c {
      'U' => Direction::Up,
      'D' => Direction::Down,
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => panic!("Invalid direction"),
    }
  }
}

#[derive(Debug)]
struct Dig<'a> {
  direction: Direction,
  distance: isize,
  color: &'a str,
}

impl Dig<'_> {
  fn decode_color(&self) -> (Direction, isize) {
    let distance = isize::from_str_radix(&self.color[..self.color.len() - 1], 16).unwrap();
    let direction = match self.color.chars().last().unwrap() {
      '3' => Direction::Up,
      '1' => Direction::Down,
      '2' => Direction::Left,
      '0' => Direction::Right,
      _ => panic!("Invalid direction"),
    };

    (direction, distance)
  }
}

#[derive(Debug)]
struct Plan<'a> {
  digs: Vec<Dig<'a>>,
}

impl Plan<'_> {
  fn parse<'a>(input: &'a str) -> Plan<'a> {
    let digs = input
      .lines()
      .map(|line| {
        let splitted = line.splitn(3, ' ').collect::<Vec<_>>();
        let direction = Direction::from(splitted[0].chars().next().unwrap());
        let distance = splitted[1].parse::<isize>().unwrap();
        let color = splitted[2];
        let color = &color[2..color.len() - 1];

        Dig {
          direction,
          distance,
          color,
        }
      })
      .collect();

    Plan { digs }
  }

  fn get_lagoon_volume(&self, start_point: (isize, isize), is_part1: bool) -> isize {

    let mut current_point = start_point;
    let mut area = 0;
    let mut outer_points = 0;
    for dig in &self.digs {

      let (direction, distance) = match is_part1 {
        true => (dig.direction, dig.distance),
        false => dig.decode_color(),
      };

      let next_point = match direction {
        Direction::Up => (current_point.0, current_point.1 - distance),
        Direction::Down => (current_point.0, current_point.1 + distance),
        Direction::Left => (current_point.0 - distance, current_point.1),
        Direction::Right => (current_point.0 + distance, current_point.1),
      };
      area += (current_point.1 * next_point.0) - (current_point.0 * next_point.1);
      outer_points += distance;

      current_point = next_point;
    }
    area = area.abs();
    
    let inside_points = (area / 2) - (outer_points/2) + 1;

    outer_points as isize + inside_points as isize
  }
}

fn main() {
  
  let input = std::fs::read_to_string("input.txt").unwrap();

  let plan = Plan::parse(&input);

  println!("Part 1: {}", plan.get_lagoon_volume((0, 0), true));
  println!("Part 2: {}", plan.get_lagoon_volume((0, 0), false));

}
