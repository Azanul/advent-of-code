use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let max_sum =
        reader
            .lines()
            .map(|line| line.unwrap())
            .fold((0, 0), |(group_sum, max_sum), line| {
                if line.is_empty() {
                    let new_max = max_sum.max(group_sum);
                    (0, new_max)
                } else {
                    let new_group_sum = group_sum + line.parse::<i32>().unwrap();
                    (new_group_sum, max_sum)
                }
            });

    println!("{}", max_sum.0.max(max_sum.1));
}
