use itertools::Itertools;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let input = include_str!("../../input/day4.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let count = (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(x, y)| input[x][y] == 'X')
        .flat_map(|(x, y)| {
            let input = input.clone();
            DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
                (0..4)
                    .map(|n| {
                        input
                            .get((x as isize + dx * n) as usize)?
                            .get((y as isize + dy * n) as usize)
                            .copied()
                    })
                    .collect::<Option<String>>()
            })
        })
        .filter(|found_word| found_word == "XMAS")
        .count() as u32;

    dbg!(count);
}
