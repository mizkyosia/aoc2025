use crate::Solution;

pub fn solve(input: String) -> Solution {
    let (res1, res2) = input.split('\n').fold((0, 0), |(r1, r2), s| {
        let jolts: Vec<_> = s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

        (r1 + pack_numbers(2, &jolts), r2 + pack_numbers(12, &jolts))
    });

    Solution(res1.to_string(), res2.to_string())
}

fn pack_numbers(n: usize, jolts: &Vec<u64>) -> u64 {
    let mut maxs = vec![0; n];
    let mut indices = vec![0; n];

    for j in 0..n {
        for i in if j == 0 { 0 } else { indices[j - 1] + 1 }..=(jolts.len() - n + j) {
            if jolts[i] > maxs[j] {
                maxs[j] = jolts[i];
                indices[j] = i;
            }
        }
    }

    maxs.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| 10u64.pow(i as u32) * n + acc)
}
