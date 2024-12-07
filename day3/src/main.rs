use regex::Regex;
use std::fs;

fn main() {
    let text = fs::read_to_string("./text.txt").unwrap();
    let reg = Regex::new(r"mul\(.{1,3},.{1,3}\)").unwrap();
    let mut results: Vec<&str> = reg.find_iter(&text).map(|m| m.as_str()).collect();
    let mut total = 0;

    let reg = Regex::new(r"[0-9]{1,3}").unwrap();
    for i in results {
        let mut nums: Vec<i32> = reg
            .find_iter(i)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        total += nums[1] * nums[0];
    }

    println!("{:?}", total);
}
