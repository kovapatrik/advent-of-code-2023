#[derive(Debug)]
struct Card {
  id: u8,
  winning_numbers: Vec<u8>,
  picked_numbers: Vec<u8>
}

fn get_cards(input: &String) -> Vec<Card> {
  let mut cards = Vec::new();
  input
    .lines()
    .enumerate()
    .for_each(|(i, line)| {
      let splitted = line.split_once(": ").unwrap();
      let card_splitted = splitted.1.split_once(" | ").unwrap();
      
      cards.push(Card {
        id: i as u8,
        winning_numbers: card_splitted.0.trim().split(" ").filter_map(|x| x.parse::<u8>().ok()).collect::<Vec<u8>>(),
        picked_numbers: card_splitted.1.trim().split(" ").filter_map(|x| x.parse::<u8>().ok()).collect::<Vec<u8>>()
      });
    });
  cards
}

fn get_matches(card: &Card) -> u32 {
  card.picked_numbers
    .iter()
    .map(|n| card.winning_numbers.contains(n) as u32)
    .sum::<u32>()
}

fn part1(cards: &Vec<Card>) -> u32 {
  cards
    .iter()
    .map(|card| {
      let matches = get_matches(card);
      if matches == 0 {
        0
      } else {
        2u32.pow((matches-1) as u32)
      }
    })
    .sum::<u32>()
}

fn part2(cards: &Vec<Card>) -> u32 {
  let mut counts = vec![1; cards.len()];
  cards
    .iter()
    .for_each(|card| {
      let matches = get_matches(card);
      for id_mod in 1..=matches {
        let idx = ((card.id as u32)+id_mod) as usize;
        counts[idx] += counts[card.id as usize];
      }
    });
  counts
    .iter()
    .sum::<u32>()
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let cards = get_cards(&input);

  println!("First: {}", part1(&cards));
  println!("Second: {}", part2(&cards));
}
