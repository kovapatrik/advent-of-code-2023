use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let colors: HashMap<&str, u8> = HashMap::from([
      ("red", 12),
      ("green", 13),
      ("blue", 14)
    ]);

    let first = input
      .lines()
      .enumerate()
      .map(|(idx, line)| {
        let splitted = line.split_once(": ").unwrap();
        let mut cubes = splitted.1.split([';', ',']);
        if cubes.all(|cube| {
          let trimmmed = cube.trim();
          let splitted = trimmmed.split_once(' ').unwrap();
          let number = splitted.0.parse::<u8>().unwrap();
          let color = splitted.1;
          colors.get(color).unwrap() >= &number
        }) {
          (idx+1) as u16
        } else {
          0
        }
      })
      .sum::<u16>();

    println!("First: {}", first);

    let second = input
      .lines()
      .map(|line| {
        let splitted = line.split_once(": ").unwrap();
        let cubes = splitted.1.split([';', ',']);

        let mut colors: HashMap<&str, u32> = HashMap::from([
          ("red", 0),
          ("green", 0),
          ("blue", 0)
        ]);

        cubes.for_each(|cube| {
          let trimmmed = cube.trim();
          let splitted = trimmmed.split_once(' ').unwrap();
          let number = splitted.0.parse::<u32>().unwrap();
          let color = splitted.1;

          if number > *colors.get(color).unwrap() {
            colors.insert(color, number);
          }
        });

        colors.values().product::<u32>()

      })
      .sum::<u32>();

    println!("Second: {}", second);
}
