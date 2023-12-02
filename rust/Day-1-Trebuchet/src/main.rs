use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let calibration_sum:u32 = file_contents
        .split("\n")
        .map(|x| {
            let fd = x.find(char::is_numeric).unwrap();
            let ld = x.chars().rev().find(|c| char::is_numeric(*c)).unwrap();
            let z = x.chars().nth(fd).unwrap().to_string() + &ld.to_string();
            z.parse::<u32>().unwrap()})
        .sum();
    
    println!("{}", calibration_sum)
}
