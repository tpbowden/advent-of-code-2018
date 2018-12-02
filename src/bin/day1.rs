use std::collections::HashSet;

fn main() {
  let input = include_str!("../../input/day1.txt");
  println!("# Day 1");
  println!("Part 1: {}", part1(input));
  println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
  return input
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .sum::<i32>();
}

fn part2(input: &str) -> i32 {
  let mut values = HashSet::new();
  let mut acc = 0;
  let list = input
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .cycle();

  for item in list {
    acc = acc + item;
    if values.contains(&acc) {
      break;
    }

    values.insert(acc);
  }
  return acc;
}
