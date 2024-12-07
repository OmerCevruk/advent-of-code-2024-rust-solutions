use std::collections;
use std::collections::VecDeque;
use std::fs;
fn part1() {
    let content = fs::read_to_string("input.txt").expect("Could not read the file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Not a valid number"));
            (
                nums.next().expect("Missing first number"),
                nums.next().expect("Missing second number"),
            )
        })
        .unzip();
    left.sort();
    right.sort();

    left.reverse();
    right.reverse();
    let mut distance: i32 = 0;
    while !(left.is_empty() || right.is_empty()) {
        let numl = left.pop().unwrap();
        let numr = right.pop().unwrap();
        distance += (numl - numr).abs();
    }

    println!("{:?}", distance);
}
fn part2() {
    let mut total = 0;
    let content = fs::read_to_string("input.txt").expect("Could not read the file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Not a valid number"));
            (
                nums.next().expect("Missing first number"),
                nums.next().expect("Missing second number"),
            )
        })
        .unzip();
    for i in left {
        let count = right.iter().filter(|num| **num == i).count();
        total += count as i32 * i;
    }
    println!("{}", total);
}
fn main() {
    // part1();
    part2();
}
