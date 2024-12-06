use itertools::Itertools;

fn main() {
    let safe = include_str!(r"../../input/day2.txt")
        .trim()
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|values| {
            values
                .windows(2)
                .all(|w| w[0] > w[1] && (w[0] - w[1] <= 3) && (w[0] - w[1] >= 1))
                || values.iter().combinations(values.len() - 1).any(|values| {
                    values
                        .windows(2)
                        .all(|w| w[0] > w[1] && (w[0] - w[1] <= 3) && (w[0] - w[1] >= 1))
                })
                || values
                    .windows(2)
                    .all(|w| w[0] < w[1] && (w[1] - w[0] <= 3) && (w[1] - w[0] >= 1))
                || values.iter().combinations(values.len() - 1).any(|values| {
                    values
                        .windows(2)
                        .all(|w| w[0] < w[1] && (w[1] - w[0] <= 3) && (w[1] - w[0] >= 1))
                })
        })
        .count();

    dbg!(safe);
}
