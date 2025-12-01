use regex::Regex;

use crate::Solution;

pub fn solve(input: String) -> Solution {
    let regex = Regex::new(r"([LR])([0-9]+)").unwrap();

    let mut dial = 50; // We just need a number congruent to 50 mod 100
    let mut res1 = 0;
    let mut res2 = 0;
    let mut prev_dial: i64 = 50;

    for m in regex.captures_iter(&input) {
        let n: i64 = m[2].parse().unwrap();

        match &m[1] {
            "L" => dial -= n,
            "R" => dial += n,
            _ => unreachable!(),
        }

        if dial.rem_euclid(100) == 0 {
            res1 += 1;
        }

        res2 += (prev_dial / 100 - dial / 100).abs();

        prev_dial = dial;
    }

    Solution(res1.to_string(), res2.to_string())
}
