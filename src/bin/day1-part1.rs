fn main() {
    let (mut left, mut right) = include_str!(r"../../input/day1.txt")
        .trim()
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>();

    dbg!(sum);
}
