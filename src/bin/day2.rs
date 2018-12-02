fn main() {
  let input = include_str!("../../input/day2.txt");
  println!("# Day 2");
  println!("Part 1: {}", part1(input));
  println!("Part 2: {}", part2(input));
}

struct TwosAndThrees(i32, i32);

fn occurrences(letter: char, word: &str) -> i32 {
  return word
    .chars()
    .fold(0, |acc, c| if c == letter { acc + 1 } else { acc });
}

#[test]
fn test_occurrences() {
  assert_eq!(2, occurrences('l', "hello"));
  assert_eq!(0, occurrences('x', "hello"));
  assert_eq!(1, occurrences('o', "hello"));
}

fn has_duplicates(id: &str, n: i32) -> bool {
  let mut has_n_duplicates = false;
  for letter in id.chars() {
    if occurrences(letter, id) == n {
      has_n_duplicates = true;
    }
  }

  return has_n_duplicates;
}

fn part1(input: &str) -> i32 {
  let result = input.lines().fold(TwosAndThrees(0, 0), |acc, id| {
    let twos = if has_duplicates(id, 2) {
      acc.0 + 1
    } else {
      acc.0
    };
    let threes = if has_duplicates(id, 3) {
      acc.1 + 1
    } else {
      acc.1
    };
    return TwosAndThrees(twos, threes);
  });

  return result.0 * result.1;
}

#[test]
fn test_part1() {
  assert_eq!(
    12,
    part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab")
  );
}

fn matching_characters(a: &str, b: &str) -> String {
  a.chars()
    .zip(b.chars())
    .filter(|chars| chars.0 == chars.1)
    .map(|chars| chars.0)
    .collect::<String>()
}

#[test]
fn test_matching_characters() {
  assert_eq!("hello", matching_characters("hello", "hello"));
  assert_eq!("llo", matching_characters("hello", "ehllo"));
  assert_eq!("", matching_characters("hello", "goodbye"));
}

fn count_mismatching_chars(a: &str, b: &str) -> i32 {
  a.chars().zip(b.chars()).fold(0, |acc, chars| {
    if chars.0 == chars.1 {
      return acc;
    } else {
      return acc + 1;
    }
  })
}

#[test]
fn test_count_mismatching_chars() {
  assert_eq!(0, count_mismatching_chars("hello", "hello"));
  assert_eq!(2, count_mismatching_chars("hello", "ehllo"));
}

fn part2(input: &str) -> String {
  for line in input.lines() {
    let result = input
      .lines()
      .find(|comparison| count_mismatching_chars(line, comparison) == 1);

    match result {
      Some(result) => return matching_characters(line, result),
      None => continue,
    }
  }
  panic!()
}

#[test]
fn test_part2() {
  assert_eq!(
    "fgij",
    part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz")
  );
}
