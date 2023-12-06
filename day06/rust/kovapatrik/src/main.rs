struct Race {
    time: usize,
    distance: usize,
}

fn get_races(input: &String, part: usize) -> Vec<Race> {

  let mut lines = input.lines();

  let times = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<usize>().unwrap());
  let distances = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<usize>().unwrap());
  
  if part == 1 {
    times
    .zip(distances)
    .map(|(t,d)| Race { time: t, distance: d })
    .collect()
  } else {
    let time = times.map(|t| t.to_string()).collect::<Vec<String>>().join("");
    let distance = distances.map(|d| d.to_string()).collect::<Vec<String>>().join("");

    let race = Race {
      time: time.parse::<usize>().unwrap(),
      distance: distance.parse::<usize>().unwrap(),
    };

    Vec::from([race])
  }
}

fn solve_race(r: &Race) -> usize {
  let disc = (r.time.pow(2) - 4 * r.distance) as f32;

  let q1 = ((((r.time as f64) - (disc.sqrt() as f64)) / 2.0).floor() as usize) + 1;
  let q2 = (((r.time as f64) + (disc.sqrt() as f64)) / 2.0).ceil() as usize;

  q2 - q1
}

fn part1(races: &Vec<Race>) -> usize {
  races
    .iter()
    .map(|r| {
      solve_race(r)
    })
    .product()
}
fn main() {

  let input = std::fs::read_to_string("input.txt").unwrap();

  let part1_races = get_races(&input, 1);

  println!("Part 1: {}", part1(&part1_races));

  let part2_races = get_races(&input, 2);

  println!("Part 2: {}", part1(&part2_races))
}
