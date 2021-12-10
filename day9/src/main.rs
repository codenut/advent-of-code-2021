use std::collections::HashSet;

const RADIX: u32 = 10;
const DX: [isize; 4] = [-1, 1, 0, 0];
const DY: [isize; 4] = [0, 0, -1, 1];

fn to_int(c: char) -> isize {
    return c.to_digit(RADIX).unwrap() as isize;
}

fn to_vec(s: &str) -> Vec<isize> {
    return s.chars().map(|c| to_int(c)).collect();
}

fn get_in_arr(v: &Vec<Vec<isize>>, i: isize, j: isize) -> isize {
    return v[i as usize][j as usize]
}

fn dfs(grid: &Vec<Vec<isize>>, a: usize, b: usize, seen: &mut HashSet<(isize, isize)>) {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    for d in 0..4 {
        let ni: isize = DX[d] + (a as isize);
        let nj: isize = DY[d] + (b as isize);

        if ni >= 0 && ni < n && nj >= 0 && nj < m
            && grid[a][b] < get_in_arr(grid, ni, nj)
            && get_in_arr(grid, ni, nj) < 9
            && !seen.contains(&(ni, nj)){

            seen.insert((ni, nj));
            dfs(grid, ni as usize, nj as usize, seen);
        }
    }
}

fn main() {
    let lines: Vec<Vec<isize>> = include_str!("../data/day9.in")
        .lines()
        .map(|s| to_vec(s))
        .collect();

    let n: isize = lines.len() as isize;
    let m: isize = lines[0].len() as isize;
    let mut basins: Vec<usize> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let mut c = 0;

            for d in 0..4 {
                let ni: isize = DX[d] + (i as isize);
                let nj: isize = DY[d] + (j as isize);

                if ni >= 0 && ni < n && nj >= 0 && nj < m {
                    if lines[i][j] < get_in_arr(&lines, ni, nj) {
                        c += 1;
                    }
                } else {
                    c += 1;
                }
            }

            if c == 4 {
                let mut seen: HashSet<(isize, isize)> = HashSet::new();
                dfs(&lines, i, j, &mut seen);
                basins.push(1 + seen.len());
            }
        }
    }

    basins.
    basins.sort();
    let prod = basins.iter().rev().take(3).fold(1, |p, v| v * p);

    println!("ans = {}", prod);
}
