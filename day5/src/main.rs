use std::cmp;


fn parse(s: &str) -> ((usize, usize), (usize, usize)) {
    let parts: Vec<&str> = s.split(" -> ").collect();
    let from: Vec<usize> = parts[0].split(",").map(|i| i.parse().unwrap()).collect();
    let to: Vec<usize> = parts[1].split(",").map(|i| i.parse().unwrap()).collect();
    return ((from[1], from[0]), (to[1], to[0]));
}

fn fill_hor(arr: &mut [[usize; 1000]; 1000], row: usize, fcol: usize, tcol: usize) {
    let a = cmp::min(fcol, tcol);
    let b = cmp::max(fcol, tcol);
    for col in a..=b {
        arr[row][col] += 1;
    }
}

fn fill_ver(arr: &mut [[usize; 1000]; 1000], col: usize, frow: usize, trow: usize) {
    let a = cmp::min(frow, trow);
    let b = cmp::max(frow, trow);
    for row in a..=b {
        arr[row][col] += 1;
    }
}

fn fill_dia(arr: &mut [[usize; 1000]; 1000], from: (usize, usize), to: (usize, usize)) {
    let (from, to) = (cmp::min(from, to), cmp::max(from, to));

    if from.1 > to.1 {
        for (i, f) in (to.1..=from.1).rev().enumerate() {
            arr[from.0 + i][f] += 1;
        }
    } else {
        for (i, f) in (from.1..=to.1).enumerate() {
            arr[from.0 + i][f] += 1;
        }
    }
}


fn main() {
    let lines: Vec<((usize, usize), (usize, usize))> = include_str!("../data/day5.in")
        .lines()
        .map(|s| parse(s))
        .collect();

    let mut arr = [[0; 1000]; 1000];
    let mut ans = 0;

    for ((frow, fcol), (trow, tcol)) in lines {
        if frow == trow {
            fill_hor(&mut arr, frow, fcol, tcol);
        } else if fcol == tcol {
            fill_ver(&mut arr, fcol, frow, trow);
        } else {
            fill_dia(&mut arr, (frow, fcol), (trow, tcol));
        }
    }


    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] > 1 {
                ans += 1;
            }
        }
    }


    println!("{}", ans);
}
