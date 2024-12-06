use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq)]
enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

fn main() {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|do\(\)|don't\(\)").unwrap();
    let instrs = re
        .captures_iter(include_str!("../../input/day3.txt"))
        .map(|m| {
            if let Some(a) = m.get(2) {
                Instr::Mul(
                    a.as_str().parse().unwrap(),
                    m.get(3).unwrap().as_str().parse().unwrap(),
                )
            } else {
                if m.get(0).unwrap().as_str() == "do()" {
                    Instr::Do
                } else {
                    Instr::Dont
                }
            }
        })
        .collect_vec();

    let mut do_instr = true;
    let mut sum = 0;

    for instr in instrs {
        if do_instr {
            match instr {
                Instr::Mul(a, b) => sum += a * b,
                Instr::Dont => do_instr = false,
                Instr::Do => {}
            }
        } else if instr == Instr::Do {
            do_instr = true;
        }
    }

    dbg!(sum);
}
