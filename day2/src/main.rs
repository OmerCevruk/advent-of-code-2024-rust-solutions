use std::fs;
fn main() {
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
