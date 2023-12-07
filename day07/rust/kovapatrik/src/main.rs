use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
enum HandType {
  HighCard = 1,
  OnePair,
  TwoPairs,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

impl Display for HandType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      HandType::HighCard => write!(f, "High Card"),
      HandType::OnePair => write!(f, "One Pair"),
      HandType::TwoPairs => write!(f, "Two Pairs"),
      HandType::ThreeOfAKind => write!(f, "Three of a Kind"),
      HandType::FullHouse => write!(f, "Full House"),
      HandType::FourOfAKind => write!(f, "Four of a Kind"),
      HandType::FiveOfAKind => write!(f, "Five of a Kind"),
    }
  }
}

#[derive(Debug)]
struct Hand {
  cards: Vec<u8>,
  typ: HandType,
  bid: u16,
}

fn get_hands(input: &String, part: u8) -> Vec<Hand> {

  input
    .lines()
    .map(|line| {
      let splitted = line.split_once(' ').unwrap();
      
      let cards = splitted.0.chars().map(|c| {
        match c.is_digit(10) {
          true => c.to_digit(10).unwrap() as u8,
          false => match c {
            'T' => 10,
            'J' => if part == 1 { 11 } else { 1 },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card: {}", c),
          }
        }
      }).collect::<Vec<u8>>();
      let bid = splitted.1.parse::<u16>().unwrap();

      let mut ordered_cards = cards.clone();
      ordered_cards.sort();
      let mut ordered_cards: VecDeque<u8> = ordered_cards.into();
      
      let mut hand_types = vec![];
      let mut jokers = 0;

      while ordered_cards.len() > 0 {
        let card = ordered_cards.pop_front().unwrap();
        
        if card == 1 {
          jokers += 1;
          continue;
        }

        let mut count = 1;

        loop {
          match ordered_cards.front() {
            Some(c) if *c == 1 => {
              jokers += 1;
              ordered_cards.pop_front();
            },
            Some(c) if *c == card => {
              count += 1;
              ordered_cards.pop_front();
            },
            _ => break,
          }
        }

        match count {
          1 => hand_types.push(HandType::HighCard),
          2 => hand_types.push(HandType::OnePair),
          3 => hand_types.push(HandType::ThreeOfAKind),
          4 => hand_types.push(HandType::FourOfAKind),
          5 => hand_types.push(HandType::FiveOfAKind),
          _ => panic!("Invalid count: {}", count),
        }
      };

      let typ = if hand_types.iter().filter(|t| **t == HandType::OnePair).count() == 2 {
        HandType::TwoPairs
      } else if hand_types.contains(&HandType::OnePair) && hand_types.contains(&HandType::ThreeOfAKind) {
        HandType::FullHouse
      } else {
        match hand_types.iter().max() {
          Some(t) => t.clone(),
          None => HandType::FiveOfAKind,
        }
      };

      let new_typ = match typ {
        HandType::HighCard => {
          match jokers {
            0 => HandType::HighCard,
            1 => HandType::OnePair,
            2 => HandType::ThreeOfAKind,
            3 => HandType::FourOfAKind,
            4 => HandType::FiveOfAKind,
            _ => panic!("Invalid jokers: {}", jokers),
          }
        },
        HandType::OnePair => match jokers {
          0 => HandType::OnePair,
          1 => HandType::ThreeOfAKind,
          2 => HandType::FourOfAKind,
          3 => HandType::FiveOfAKind,
          _ => panic!("Invalid jokers: {}", jokers),
        },
        HandType::TwoPairs => match jokers {
          0 => HandType::TwoPairs,
          1 => HandType::FullHouse,
          _ => panic!("Invalid jokers: {}", jokers),
        },
        HandType::ThreeOfAKind => match jokers {
          0 => HandType::ThreeOfAKind,
          1 => HandType::FourOfAKind,
          2 => HandType::FiveOfAKind,
          _ => panic!("Invalid jokers: {}", jokers),
        },
        HandType::FullHouse => HandType::FullHouse,
        HandType::FourOfAKind => match jokers {
          0 => HandType::FourOfAKind,
          1 => HandType::FiveOfAKind,
          _ => panic!("Invalid jokers: {}", jokers),
        },
        HandType::FiveOfAKind => HandType::FiveOfAKind,
      };
      
      Hand {
        cards,
        bid,
        typ: new_typ
      }
    })
    .collect()
}

fn solve_bids(mut hands: Vec<Hand>) -> usize {
  hands.sort_by(|a,b| b.typ.cmp(&a.typ).then(b.cards.cmp(&a.cards)));

  let length = hands.len();

  hands
    .iter()
    .enumerate()
    .map(|(i, h)| {
      h.bid as usize * (length - i)
    })
    .sum()
}
fn main() {
    
  let input = std::fs::read_to_string("input.txt").unwrap();

  let hands = get_hands(&input, 1);
  println!("Part 1: {}", solve_bids(hands));

  let hands = get_hands(&input, 2);
  println!("Part 2: {}", solve_bids(hands));
}
