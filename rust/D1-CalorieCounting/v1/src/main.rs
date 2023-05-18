use std::fs;

fn main() {
    let file_contents = fs::read_to_string("../input.txt").unwrap();

    let max_sum = file_contents
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|element| element.parse().unwrap_or(0))
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{}", max_sum)
}
