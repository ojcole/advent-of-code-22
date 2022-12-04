use std::{collections::HashSet, fs};

fn split_mid(line: &str) -> (String, String) {
    let parts = line.split_at(line.len() / 2);
    (parts.0.to_string(), parts.1.to_string())
}

fn load_input_1() -> Vec<(String, String)> {
    let contents = fs::read_to_string("./inputs/day3.txt").expect("file not found");
    contents.lines().map(split_mid).collect::<Vec<_>>()
}

fn load_input_2() -> Vec<String> {
    let contents = fs::read_to_string("./inputs/day3.txt").expect("file not found");
    contents
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn score_letter(c: char) -> i32 {
    let num = c as i32;
    if c.is_uppercase() {
        num - 'A' as i32 + 27
    } else {
        num - 'a' as i32 + 1
    }
}

#[allow(dead_code)]
pub fn part1() {
    let mut total = 0;
    let input = load_input_1();
    for (left, right) in input {
        let left_set = left.chars().collect::<HashSet<char>>();
        let right_set = right.chars().collect::<HashSet<char>>();
        let in_both = left_set.intersection(&right_set);
        for item in in_both {
            total += score_letter(*item);
        }
    }
    println!("{total}");
}

#[allow(dead_code)]
pub fn part2() {
    let mut total = 0;
    let input = load_input_2();
    for i in (0..input.len()).step_by(3) {
        let set1 = input[i].chars().collect::<HashSet<char>>();
        let set2 = input[i + 1].chars().collect::<HashSet<char>>();
        let set3 = input[i + 2].chars().collect::<HashSet<char>>();
        let in_both: HashSet<char> = &set1 & &set2;
        let c = in_both.intersection(&set3).next().unwrap();
        total += score_letter(*c);
    }
    println!("{total}");
}
