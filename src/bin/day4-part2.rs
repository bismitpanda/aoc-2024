use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day4.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut count = 0;

    for x in 1..input[0].len() - 1 {
        for y in 1..input.len() - 1 {
            if input[y][x] == 'A' {
                let ul = input[y - 1][x - 1] as u8;
                let ur = input[y - 1][x + 1] as u8;
                let dl = input[y + 1][x - 1] as u8;
                let dr = input[y + 1][x + 1] as u8;

                count += (ul.abs_diff(dr) == 6 && ur.abs_diff(dl) == 6) as u32;
            }
        }
    }

    dbg!(count);
}
