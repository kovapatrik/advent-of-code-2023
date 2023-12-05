use std::{collections::{HashMap, VecDeque}, ops::Range, cmp};

#[derive(Debug)]
struct Map {
  to: String,
  destination_range: Vec<Range<usize>>,
  source_range: Vec<Range<usize>>,
}

type SeedMap = HashMap<String, Map>;

fn get_maps(input: &String) -> SeedMap {
  let mut seed_map = SeedMap::new();

  input
    .split("\n\n")
    .skip(1)
    .for_each(|line| {
      let map_lines = line.split('\n').collect::<Vec<&str>>();
      let from_to = map_lines[0].trim().to_string();
      let from_to = from_to[0..from_to.len() - 1 - 4].split_once("-to-").unwrap();
      let from = from_to.0;
      let to = from_to.1;

      let mut destination_range = Vec::new();
      let mut source_range = Vec::new();

      map_lines[1..].iter().filter(|line| line.len() > 0).for_each(|line| {
        let range = line.splitn(3, ' ').collect::<Vec<&str>>();

        let dest_start = range[0].parse::<usize>().unwrap();
        let source_start = range[1].parse::<usize>().unwrap();
        let length = range[2].parse::<usize>().unwrap();

        destination_range.push(dest_start..dest_start + length);
        source_range.push(source_start..source_start + length);
      });

      seed_map.insert(
        from.to_string(),
        Map {
          to: to.to_string(),
          destination_range,
          source_range,
        },
      );
    });

  seed_map
}

fn part1(seeds: &Vec<usize>, seed_map: &SeedMap) -> usize {
  seeds
    .iter()
    .map(|seed| {
      let mut name = "seed";
      let mut value = seed.clone();
      while name != "location" {
        let map = seed_map.get(name).unwrap();
        value = match map.source_range
          .iter()
          .zip(map.destination_range.iter())
          .find(|(source_range, _)| source_range.contains(&value)) {
          Some((source_range, dest_range)) => dest_range.start + value - source_range.start,
          None => value
          };
        name = &map.to;
      }
      value
    })
    .min().unwrap()
}

fn part2(seed_ranges: &Vec<Range<usize>>, seed_map: &SeedMap) -> usize {
  seed_ranges
    .iter()
    .map(|seed_range| {
      let mut name = "seed";

      let mut to_process = VecDeque::from([seed_range.clone()]);
      let mut processed: Vec<Range<usize>> = Vec::new();

      while name != "location" {
        let map = seed_map.get(name).unwrap();
        while let Some(range) = to_process.pop_front() {
          match map.source_range
            .iter()
            .zip(map.destination_range.iter())
            .find(|(source_range, _)| {
              source_range.contains(&range.start)
            }) {
              Some((source_range, dest_range)) => {
                let matching_range = range.start..cmp::min(range.end, source_range.end);
                let matching_dest_range = dest_range.start + matching_range.start - source_range.start..dest_range.start + matching_range.end - source_range.start;

                processed.push(matching_dest_range);

                if matching_range.end < range.end {
                  to_process.push_back(matching_range.end..range.end);
                }
                if matching_range.start > range.start {
                  to_process.push_back(range.start..matching_range.start);
                }
              },
              None => {
                processed.push(range);
              }
            }
        }
        to_process = VecDeque::from(processed.clone());
        processed.clear();
        name = &map.to;
      }
      to_process.iter().map(|range| range.start).min().unwrap()
    })
    .min().unwrap()
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let seeds = input.lines().next().unwrap();
  let seed_map = get_maps(&input);

  let part1_seeds = seeds[7..].split(' ').map(|seed| seed.parse::<usize>().unwrap()).collect::<Vec<usize>>();
  println!("Part 1: {}", part1(&part1_seeds, &seed_map));

  let part2_seeds = part1_seeds
    .chunks(2)
    .map(|chunk| (chunk[0]..chunk[0]+chunk[1]))
    .collect::<Vec<Range<usize>>>();

  println!("Part 2: {}", part2(&part2_seeds, &seed_map));
}
