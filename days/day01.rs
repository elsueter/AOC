use crate::return_sol;
use crate::utils;

fn euc_mod(mut val: i32, modulo: i32) -> i32 {
    val %= modulo;
    if val < 0 {
        val += modulo;
    }
    val
}

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 50;
    let mut count = 0;
    for line in lines {
        let dir = line.chars().next().unwrap();
        let mut val: i32 = line[1..].parse().unwrap();

        if dir == 'L' {
            val *= -1;
        }

        tot += val;

        tot = euc_mod(tot, 100);

        if tot == 0 {
            count += 1;
        }
    }

    return_sol!(count)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 50;
    let mut count = 0;
    for line in lines {
        let dir = line.chars().next().unwrap();
        let mut val: i32 = line[1..].parse().unwrap();

        count += val / 100;
        val %= 100;

        match dir {
            'L' => {
                if tot - val < 0 && tot != 0 {
                    count += 1;
                }
                val *= -1;
            }
            'R' => {
                if tot + val > 100 {
                    count += 1;
                }
            }
            _ => (),
        };

        tot += val;

        tot = euc_mod(tot, 100);

        if tot == 0 {
            count += 1;
        }
    }

    return_sol!(count)
}
