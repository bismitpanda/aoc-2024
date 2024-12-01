use counter::Counter;

fn main() {
    let (left, counter) = include_str!(r"../../input/day1.txt")
        .trim()
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unzip::<_, _, Vec<_>, Counter<_>>();

    let sum = left
        .into_iter()
        .map(|right| right * counter[&right])
        .sum::<usize>();

    dbg!(sum);
}
