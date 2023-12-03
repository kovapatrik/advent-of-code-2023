use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number {
  x: i16,
  y: i16,
  length: i16,
  value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol {
  x: i16,
  y: i16,
  value: char,
}

struct PartNumber {
  symbol: Symbol,
  number: Number,
}

type Map = HashMap<Symbol, HashSet<Number>>;

fn find_symbols(lines: &Vec<&str>) -> Vec<Symbol> {
  let mut symbols = Vec::new();
  for (x, l) in lines.iter().enumerate() {
    for (y, c) in l.chars().enumerate() {
      if c.ne(&'.') && c.is_ascii_punctuation() {
        symbols.push(Symbol {
          x: x as i16,
          y: y as i16,
          value: c,
        });
      }
    }
  }
  symbols
}

fn find_numbers(lines: &Vec<&str>) -> Vec<Number> {
  let mut numbers = Vec::new();
  let line_reg = Regex::new(r"\d+").unwrap();
  for (x, l) in lines.iter().enumerate() {
    line_reg.find_iter(l).for_each(|m| {
      let num_str = m.as_str();
      numbers.push(Number {
        x: x as i16,
        y: m.start() as i16,
        length: num_str.len() as i16,
        value: num_str.parse::<u32>().unwrap(),
      });
    });
  }
  numbers
}

fn find_part_numbers(symbols: &Vec<Symbol>, numbers: &Vec<Number>) -> Vec<PartNumber> {
  let mut part_numbers = Vec::new();
  for number in numbers {
    let x_range = number.x-1..=number.x+1;
    let y_range = number.y-1..=number.y+number.length;
    if let Some(symbol) = symbols.iter().find(|s| x_range.contains(&s.x) && y_range.contains(&s.y)) {
      part_numbers.push(PartNumber {
        symbol: *symbol,
        number: *number,
      });
    }
  }
  part_numbers
}

fn part1(part_numbers: &Vec<PartNumber>) -> u32 {
  part_numbers
    .iter()
    .map(|pn| pn.number.value)
    .sum::<u32>()
}

fn part2(part_numbers: &Vec<PartNumber>) -> u32 {
  let mut map: Map = HashMap::new();
  for pn in part_numbers {
    map.entry(pn.symbol)
      .or_insert(HashSet::new())
      .insert(pn.number);
  }
  map.values()
    .filter(|v| v.len() == 2)
    .map(|v| v.iter().map(|n| n.value).product::<u32>())
    .sum::<u32>()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let symbols = find_symbols(&lines);
    let numbers = find_numbers(&lines);
    let part_numbers = find_part_numbers(&symbols, &numbers);

    println!("Part 1: {}", part1(&part_numbers));
    println!("Part 2: {}", part2(&part_numbers));
}
