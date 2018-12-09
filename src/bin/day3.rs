extern crate regex;
use regex::Regex;

fn main() {
  let input = include_str!("../../input/day3.txt");
  println!("# Day 3");
  println!("Part 1: {}", part1(input));
  // println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Section {
  id: i32,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
}

#[test]
fn test_parse_row() {
  let result = parse_row("#1 @ 2,3: 4x5");
  assert_eq!(1, result.id);
  assert_eq!(2, result.x);
  assert_eq!(3, result.y);
  assert_eq!(4, result.width);
  assert_eq!(5, result.height);
}

fn parse_row(row: &str) -> Section {
  let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
  let captures = re.captures(row).unwrap();
  Section {
    id: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
    x: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    y: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
    width: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    height: captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
  }
}

fn part1(input: &str) -> i32 {
  let mut grid = [[0; 1000]; 1000];
  let mut counter = 0;
  for section in input.lines().map(parse_row) {
    for x in 0..section.width {
      for y in 0..section.height {
        if grid[(x + section.x) as usize][(y + section.y) as usize] == 1 {
          counter = counter + 1;
        }
        grid[(x + section.x) as usize][(y + section.y) as usize] += 1;
      }
    }
  }
  for section in input.lines().map(parse_row) {
    let mut winner = true;
    for x in 0..section.width {
      for y in 0..section.height {
        if grid[(x + section.x) as usize][(y + section.y) as usize] != 1 {
          winner = false;
        }
      }
    }
    if (winner) {
      println!("{}", section.id);
    }
  }
  counter
}

#[test]
fn test_part1() {
  assert_eq!(4, part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
}
