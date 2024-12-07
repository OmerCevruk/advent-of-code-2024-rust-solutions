use std::fs;
fn part1() {
    let file = fs::read_to_string("text.txt").expect("fuck you");
    let lines: Vec<&str> = file.split("\n").collect();
    let mut crazy: Vec<Vec<u8>> = Vec::new();

    for i in lines {
        let a: Vec<u8> = i
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();
        crazy.push(a);
    }
    crazy.retain(|v| !v.is_empty());

    let mut count = 0;
    'outer: for line in crazy {
        let mut is_increasing: bool = false;
        let mut is_decreasing: bool = false;
        for i in 0..line.len() - 1 {
            // since this is not a production code using unwrap is permissible
            let num = line.get(i).unwrap();
            let next = line.get(i + 1).unwrap();

            match num.cmp(next) {
                std::cmp::Ordering::Greater => {
                    if is_decreasing {
                        continue 'outer;
                    }
                    is_increasing = true;
                }
                std::cmp::Ordering::Less => {
                    if is_increasing {
                        continue 'outer;
                    }
                    is_decreasing = true;
                }
                std::cmp::Ordering::Equal => {
                    continue 'outer;
                }
            }

            if next.abs_diff(*num) > 3 {
                continue 'outer;
            }
        }
        if is_increasing || is_decreasing {
            count += 1;
        }
    }
    println!("{}", count);
}

// part 2

fn check_valid(line: &Vec<u8>) -> bool {
    let mut is_increasing: bool = false;
    let mut is_decreasing: bool = false;
    for i in 0..line.len() - 1 {
        // since this is not a production code using unwrap is permissible
        let num = line.get(i).unwrap();
        let next = line.get(i + 1).unwrap();

        match num.cmp(next) {
            std::cmp::Ordering::Greater => {
                if is_decreasing {
                    return false;
                }
                is_increasing = true;
            }
            std::cmp::Ordering::Less => {
                if is_increasing {
                    return false;
                }
                is_decreasing = true;
            }
            std::cmp::Ordering::Equal => {
                return false;
            }
        }

        if next.abs_diff(*num) > 3 {
            return false;
        }
    }
    is_increasing || is_decreasing
}

fn part2() {
    let file = fs::read_to_string("text.txt").expect("fuck you");
    let lines: Vec<&str> = file.split("\n").collect();
    let mut crazy: Vec<Vec<u8>> = Vec::new();

    for i in lines {
        let a: Vec<u8> = i
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();
        crazy.push(a);
    }
    crazy.retain(|v| !v.is_empty());
    let mut count = 0;

    for line in &crazy {
        if check_valid(line) {
            count += 1;
            continue;
        }

        let len = line.len();
        let mut found_valid = false;
        for i in 0..len {
            let mut modified = line.clone();
            modified.remove(i);
            if check_valid(&modified) {
                found_valid = true;
                break;
            }
        }
        if found_valid {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    //part1();
    part2();
}

//used ai to quickly generate tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_increasing() {
        assert!(check_valid(&vec![1, 2, 4, 5])); // Valid: increasing with diff <= 3
        assert!(check_valid(&vec![1, 3, 4, 6])); // Valid: increasing with diff <= 3
        assert!(!check_valid(&vec![1, 5, 7, 8])); // Invalid: first diff > 3
    }

    #[test]
    fn test_basic_decreasing() {
        assert!(check_valid(&vec![7, 6, 4, 2])); // Valid: decreasing with diff <= 3
        assert!(check_valid(&vec![9, 7, 5, 3])); // Valid: decreasing with diff <= 3
        assert!(!check_valid(&vec![9, 5, 4, 2])); // Invalid: first diff > 3
    }

    #[test]
    fn test_direction_changes() {
        assert!(!check_valid(&vec![1, 3, 2, 4])); // Invalid: changes direction
        assert!(!check_valid(&vec![5, 3, 4, 2])); // Invalid: changes direction
        assert!(!check_valid(&vec![1, 2, 1, 2])); // Invalid: changes direction multiple times
    }

    #[test]
    fn test_equal_numbers() {
        assert!(!check_valid(&vec![1, 2, 2, 3])); // Invalid: has equal adjacent numbers
        assert!(!check_valid(&vec![5, 4, 4, 3])); // Invalid: has equal adjacent numbers
        assert!(!check_valid(&vec![1, 1, 1, 1])); // Invalid: all numbers equal
    }

    #[test]
    fn test_edge_cases() {
        assert!(check_valid(&vec![1, 2])); // Valid: two numbers increasing
        assert!(check_valid(&vec![2, 1])); // Valid: two numbers decreasing
        assert!(!check_valid(&vec![1])); // Invalid: single number
        assert!(check_valid(&vec![1, 2, 3, 4, 5])); // Valid: longer sequence increasing
        assert!(check_valid(&vec![5, 4, 3, 2, 1])); // Valid: longer sequence decreasing
    }

    #[test]
    fn test_problem_examples() {
        assert!(check_valid(&vec![7, 6, 4, 2, 1])); // Safe from example
        assert!(!check_valid(&vec![1, 2, 7, 8, 9])); // Unsafe from example
        assert!(!check_valid(&vec![9, 7, 6, 2, 1])); // Unsafe from example
        assert!(!check_valid(&vec![1, 3, 2, 4, 5])); // Unsafe from example
        assert!(!check_valid(&vec![8, 6, 4, 4, 1])); // Unsafe from example
        assert!(check_valid(&vec![1, 3, 6, 7, 9])); // Safe from example
    }
}
