use crate::Solution;

pub fn solve(input: String) -> Solution {
    let (res1, mut map) = remove_rolls(
        input
            .split('\n')
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '@' => 1,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    );

    let mut res2 = res1;
    let mut nb = 1;

    while nb > 0 {
        (nb, map) = remove_rolls(map);
        res2 += nb;
    }

    Solution(res1.to_string(), res2.to_string())
}

fn remove_rolls(mut map: Vec<Vec<u8>>) -> (u16, Vec<Vec<u8>>) {
    let mut nb = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                continue;
            }
            let mut neighbors = 0;
            for dy in 0..3 {
                for dx in 0..3 {
                    if y + dy > 0 && x + dx > 0 && (dx != 1 || dy != 1) {
                        let ny = y + dy - 1;
                        let nx = x + dx - 1;
                        if ny < map.len() && nx < map[0].len() {
                            neighbors += map[ny][nx];
                        }
                    }
                }
            }
            if neighbors < 4 {
                nb += 1;
                map[y][x] = 0;
            }
        }
    }
    (nb, map)
}
