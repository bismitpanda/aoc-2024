use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let muls = re
        .captures_iter(include_str!("../../input/day3.txt"))
        .map(|m| {
            let (_, [a, b]) = m.extract::<2>();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();

            a * b
        })
        .sum::<i32>();

    dbg!(muls);
}
