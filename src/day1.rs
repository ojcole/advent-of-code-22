use std::fs;

fn load_input() -> Vec<i32> {
    let contents = fs::read_to_string("./inputs/day1.txt").expect("file not found");
    let lines = contents.lines();
    let mut vec = Vec::new();
    vec.push(0);
    for line in lines {
        if line.is_empty() {
            vec.push(0);
            continue;
        }
        let num = line.parse::<i32>().unwrap();
        if let Some(n) = vec.last_mut() {
            *n += num
        }
    }
    vec
}

#[allow(dead_code)]
pub fn part1() {
    let vec = load_input();
    let max_value = *vec.iter().max().unwrap();
    println!("{max_value}");
}

#[allow(dead_code)]
pub fn part2() {
    let mut vec = load_input();
    vec.sort();
    vec.reverse();
    let total = vec[0] + vec[1] + vec[2];
    println!("{total}");
}
