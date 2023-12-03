use std::collections::HashMap;

struct Game {
  id: u16,
  rounds: Vec<Round>,
}

struct Round {
  red: u8,
  green: u8,
  blue: u8,
}

fn get_games(input: &String) -> Vec<Game> {
  let mut games = Vec::new();
  input.lines().enumerate().for_each(|(idx, line)| {
    let splitted = line.split_once(": ").unwrap();
    let line_rounds = splitted.1.split(';');
    let mut rounds = Vec::new();
    line_rounds.for_each(|r| {
      let trimmmed = r.trim();
      let splitted = trimmmed.split(',');
      let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
      };
      splitted.for_each(|cube| {
        let trimmmed = cube.trim();
        let splitted = trimmmed.split_once(' ').unwrap();
        let number = splitted.0.parse::<u8>().unwrap();
        let color = splitted.1;
        match color {
          "red" => round.red = number,
          "green" => round.green = number,
          "blue" => round.blue = number,
          _ => panic!("Unknown color"),
        }
      });
      rounds.push(round);
    });
    games.push(Game {
      id: (idx+1) as u16,
      rounds,
    });
  });
  games
}

fn part1(games: &Vec<Game>) -> u16 {
  games
    .iter()
    .filter_map(|game| {
      if game.rounds.iter().all(|round| {
        round.red <= 12 && round.green <= 13 && round.blue <= 14
      }) {
        Some(game.id)
      } else {
        None
      }
    })
    .sum::<u16>()
}

fn part2(games: &Vec<Game>) -> u32 {
  games
    .iter()
    .map(|game| {
      let mut red = 0;
      let mut green = 0;
      let mut blue = 0;

      game.rounds.iter().for_each(|round| {
        red = red.max(round.red);
        green = green.max(round.green);
        blue = blue.max(round.blue);
      });

      red as u32 * green as u32 * blue as u32
    })
    .sum::<u32>()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let games = get_games(&input);

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}
