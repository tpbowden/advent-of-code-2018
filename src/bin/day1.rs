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

#[test]
fn test_part1() {
  assert_eq!(3, part1("+1\n+1\n+1"));
  assert_eq!(0, part1("+1\n+1\n-2"));
  assert_eq!(-6, part1("-1\n-2\n-3"));
}

fn part2(input: &str) -> i32 {
  let mut values = HashSet::new();
  let mut acc = 0;
  let list = input
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .cycle();

  for item in list {
    values.insert(acc);
    acc = acc + item;
    if values.contains(&acc) {
      break;
    }
  }
  return acc;
}

#[test]
fn test_part_2() {
  assert_eq!(0, part2("+1\n-1"));
  assert_eq!(10, part2("+3\n+3\n+4\n-2\n-4"));
  assert_eq!(5, part2("-6\n+3\n+8\n+5\n-6"));
  assert_eq!(14, part2("+7\n+7\n-2\n-7\n-4"));
}
