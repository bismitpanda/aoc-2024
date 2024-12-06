use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let (rules_str, updates) = include_str!("../../input/day5.txt")
        .split_once("\n\n")
        .unwrap();

    let mut rules = HashMap::<usize, HashSet<usize>>::new();

    for line in rules_str.lines() {
        let (before, after) = line.split_once('|').unwrap();
        rules
            .entry(after.parse().unwrap())
            .or_default()
            .insert(before.parse().unwrap());
    }

    let count = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect_vec()
        })
        .filter_map(|update| {
            update
                .is_sorted_by(|a, b| rules[b].contains(a))
                .then_some(update[update.len() / 2])
        })
        .sum::<usize>();

    dbg!(count);
}
