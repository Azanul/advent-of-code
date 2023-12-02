use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let calibration_sum:u32 = file_contents
        .split("\n")
        .map(|x| {
            let nums = find_numbers(&x);
            println!("{:?}", nums);
            nums.first().unwrap() * 10 + nums.last().unwrap()})
        .sum();
    
    println!("{}", calibration_sum)
}

fn find_numbers(x: &str) -> Vec<u32> {
    let mut nums : Vec<u32> = [].to_vec();
    for (i, c) in x.char_indices() {
        if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap());
        } else  {
            let z = is_num(x.get(i..).unwrap());
            if z == -1 {
                continue
            }
            nums.push(z.unsigned_abs());
        }
    }
    nums
}

fn is_num(x: &str) -> i32 {
    match x {
        s if s.starts_with("one") => 1,
        s if s.starts_with("two") => 2,
        s if s.starts_with("three") => 3,
        s if s.starts_with("four") => 4,
        s if s.starts_with("five") => 5,
        s if s.starts_with("six") => 6,
        s if s.starts_with("seven") => 7,
        s if s.starts_with("eight") => 8,
        s if s.starts_with("nine") => 9,
        _ => -1,
    }
}