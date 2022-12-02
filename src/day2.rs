use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn load_input() -> Vec<(Choice, Choice)> {
    let contents = fs::read_to_string("./inputs/day2.txt").expect("file not found");
    let mut strategy = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let p1 = if parts[0] == "A" {
            Choice::Rock
        } else if parts[0] == "B" {
            Choice::Paper
        } else {
            Choice::Scissors
        };

        let p2 = if parts[1] == "X" {
            Choice::Rock
        } else if parts[1] == "Y" {
            Choice::Paper
        } else {
            Choice::Scissors
        };

        strategy.push((p1, p2));
    }
    strategy
}

#[allow(dead_code)]
pub fn part1() {
    let input = load_input();
    let mut score = 0;
    for (them, me) in input {
        score += me as i32;
        if them == me {
            score += 3
        } else if (them == Choice::Rock && me == Choice::Paper)
            || (them == Choice::Paper && me == Choice::Scissors)
            || (them == Choice::Scissors && me == Choice::Rock)
        {
            score += 6
        }
    }
    println!("{score}")
}

fn loses(choice: Choice) -> Choice {
    match choice {
        Choice::Rock => Choice::Scissors,
        Choice::Paper => Choice::Rock,
        Choice::Scissors => Choice::Paper,
    }
}

fn wins(choice: Choice) -> Choice {
    match choice {
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissors,
        Choice::Scissors => Choice::Rock,
    }
}

#[allow(dead_code)]
pub fn part2() {
    let input = load_input();
    let mut score = 0;
    for (them, me) in input {
        match me {
            Choice::Rock => score += loses(them) as i32,
            Choice::Paper => score += them as i32 + 3,
            Choice::Scissors => score += wins(them) as i32 + 6,
        }
    }
    println!("{score}")
}
