use std::ops::RangeInclusive;

use crate::Solution;

fn combine_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut output = Vec::with_capacity(ranges.len());

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    output.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let j = output.len() - 1;

        if ranges[i].0 >= output[j].0 && ranges[i].0 <= output[j].1 {
            output[j].1 = output[j].1.max(ranges[i].1)
        } else {
            output.push(ranges[i].clone());
        }
    }

    output
}

pub fn solve(input: String) -> Solution {
    let input = input
        .split("\n\n")
        .map(|l| l.split('\n').collect())
        .collect::<Vec<Vec<_>>>();

    let ranges: Vec<(u64, u64)> = input[0]
        .iter()
        .map(|l| {
            let v: Vec<_> = l.split('-').collect();
            (v[0].parse().unwrap(), v[1].parse().unwrap())
        })
        .collect();

    let res1: u64 = input[1]
        .iter()
        .map(|l| {
            let n: u64 = l.parse().unwrap();

            for (a, b) in ranges.iter() {
                if n >= *a && n <= *b {
                    return 1;
                }
            }
            0
        })
        .sum();

    let mut res2 = 0;

    let test = combine_ranges(ranges);

    println!("{:?}", test);

    for (a, b) in test {
        res2 += b - a + 1;
    }


    Solution(res1.to_string(), res2.to_string())
}
