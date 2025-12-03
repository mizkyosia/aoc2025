use std::collections::{HashMap, HashSet};

use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut set: HashSet<i64> = HashSet::new();
    let mut set_2: HashSet<i64> = HashSet::new();

    input.split(',').for_each(|l| {
        let tmp = l
            .split('-')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        for i in tmp[0]..=tmp[1] {
            if set.contains(&i) {
                continue;
            }
            //  println!("{i}");

            let n = 1 + (i as f32).log10() as i64;
            let exp = 10i64.pow(n as u32 / 2);
            let a = i / exp;
            let b = i % exp;

            // Part 1
            if a == b {
                set.insert(i);
                set_2.insert(i);
            }

            // Part 2
            for d in 1..n {
                // Check if d is divisor of n
                if n % d == 0 {
                    //  println!("  {n} : {d}");
                    if let Some((_, true)) = (0..((n / d) as u32))
                        .map(|j| {
                            let exp_lo = 10i64.pow(d as u32 * j);
                            let exp_hi = 10i64.pow(d as u32 * (j+1));

                            //  println!("      {exp_lo}, {exp_hi}, {}", (i % exp_hi) / exp_lo);

                            ((i % exp_hi) / exp_lo, true)
                        })
                        .reduce(|(acc_n, v), (n, _)| (acc_n, acc_n == n && v))
                    {
                        set_2.insert(i);
                        // println!(":3");
                        break;
                    }
                }
            }
        }
    });

    Solution(
        set.iter().sum::<i64>().to_string(),
        set_2.iter().sum::<i64>().to_string(),
    )
}
