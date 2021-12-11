use std::collections::{HashMap, HashSet};

fn adjacents(row: i32, col: i32) -> [(i32, i32); 8] {
    return [
        (row, col + 1), (row, col - 1),
        (row + 1, col), (row - 1, col),
        (row - 1, col - 1), (row - 1, col + 1),
        (row + 1, col + 1), (row + 1, col - 1),
    ];
}

fn valid_adjacents(row: i32, col: i32, n: i32, m: i32) -> Vec<(i32, i32)> {
    return adjacents(row, col).iter().filter(|(row, col)| {
        *row >= 0 && *row < n && *col >= 0 && *col < m
    }).map(|(row, col)| (*row, *col))
        .collect();
}

fn simulate(dim_to_energy: &mut HashMap<(i32, i32), i32>, n: i32, m: i32) -> i32 {
    let mut flashed: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            let energy = *dim_to_energy.get(&(i, j)).unwrap();
            dim_to_energy.insert((i, j), energy + 1);

            if energy + 1 == 10 {
                flashed.push((i, j));
            }
        }
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    while flashed.len() > 0 {
        let (row, col) = flashed.pop().unwrap();
        dim_to_energy.insert((row, col), 0);

        if seen.contains(&(row, col)) {
            continue;
        }

        seen.insert((row, col));

        for (nrow, ncol) in valid_adjacents(row, col, n, m) {
            let energy = *dim_to_energy.get(&(nrow, ncol)).unwrap();
            if energy < 10 && !seen.contains(&(nrow, ncol)) {
                dim_to_energy.insert((nrow, ncol), energy + 1);
                if energy + 1 == 10 {
                    flashed.push((nrow, ncol));
                }
            }
        }
    }

    return seen.len() as i32;
}


fn main() {
    let lines: Vec<Vec<usize>> = include_str!("../data/day11.in")
        .lines()
        .map(|line|
            line.chars().map(|c| c.to_digit(10).unwrap() as usize)
                .collect())
        .collect();

    let n = lines.len();
    let m = lines[0].len();

    let mut dim_to_energy: HashMap<(i32, i32), i32> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            dim_to_energy.insert((i as i32, j as i32), lines[i][j] as i32);
        }
    }

    let mut count = 0;
    for i in 0..1000 {
        let c = simulate(&mut dim_to_energy, n as i32, m as i32);
        if c as usize == n * m {
            println!("all flashed={}", i + 1);
            break;
        }
        count += c;
    }

    println!("total flashed={}", count);
}
