use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: i64,
    y: i64,
}

fn get_expands(input: &String) -> (Vec<i64>, Vec<i64>) {
  let mut x: Vec<i64> = Vec::new();
  let mut y: Vec<i64> = Vec::new();
  input
    .lines()
    .enumerate()
    .filter(|(_, line)| !line.contains("#"))
    .for_each(|(i, _)| {
      y.push(i as i64);
    });
  
  let len = input.lines().nth(0).unwrap().len();
  let mut iters = input.lines().map(|l| l.chars()).collect::<Vec<_>>();
  (0..len)
    .for_each(|j| {
      let t = iters.iter_mut().map(|iter| iter.next().unwrap()).collect::<Vec<_>>();
      if !t.contains(&'#') {
        x.push(j as i64);
      }
    });
  (x, y)
}

fn get_galaxies(input: &String) -> Vec<Galaxy> {
  input
    .lines()
    .enumerate()
    .flat_map(|(i, line)| {
      line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(j, _)| Galaxy { x: j as i64, y: i as i64 })
        .collect::<Vec<Galaxy>>()
    })
    .collect::<Vec<Galaxy>>()
}

fn solve(galaxies: &Vec<Galaxy>, x_expands: &Vec<i64>, y_expands: &Vec<i64>, part: usize) -> i64 {

  galaxies
    .iter()
    .combinations(2)
    .map(|g| {
      let (mut g1, mut g2) = (g[0].clone(), g[1].clone());

      g1.x += (x_expands.iter().filter(|x| **x < g1.x).count() * (if part == 1 { 1 } else { 1000000-1 })) as i64;
      g1.y += (y_expands.iter().filter(|y| **y < g1.y).count() * (if part == 1 { 1 } else { 1000000-1 })) as i64;
      
      g2.x += (x_expands.iter().filter(|x| **x < g2.x).count() * (if part == 1 { 1 } else { 1000000-1 })) as i64;
      g2.y += (y_expands.iter().filter(|y| **y < g2.y).count() * (if part == 1 { 1 } else { 1000000-1 })) as i64;

      let x = (g1.x - g2.x).abs();
      let y = (g1.y - g2.y).abs();

      x+y
    })
    .sum()
}

fn main() {
    
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (x_expands, y_expands) = get_expands(&input);
    let galaxies = get_galaxies(&input);

    println!("Part 1: {}", solve(&galaxies, &x_expands, &y_expands, 1));
    println!("Part 2: {}", solve(&galaxies, &x_expands, &y_expands, 2));
}
