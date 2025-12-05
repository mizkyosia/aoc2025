use crate::Solution;
use crate::transform::Transform;

pub fn solve(input: String) -> Solution {
    let (res1, res2, _) = input
        .split('\n')
        .map(|l| {
            l.split_at(1).transform(|(s, n)| {
                (match *s {
                    "R" => 1,
                    "L" => -1,
                    _ => 0,
                }) * n.parse::<i64>().unwrap()
            })
        })
        .fold((0, 0, 50), |(a, b, d), n| {
            println!(
                "{}    {}      {}",
                (d + n),
                d,
                (d + n) / 100 - d / 100
            );
            (
                a + if (d + n).rem_euclid(100) == 0 { 1 } else { 0 },
                b + ((d + n) / 100 - d / 100).abs(),
                d + n,
            )
        });

    Solution(res1.to_string(), res2.to_string())
}
