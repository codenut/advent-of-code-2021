fn sum(n: isize) -> isize {
    return (n * (n + 1)) / 2;
}

fn brute_force(crabs: &Vec<isize>) -> isize {
    let mut ans = std::isize::MAX;
    let n = crabs.len();
    let (a, b) = (*crabs.iter().min().unwrap(), *crabs.iter().max().unwrap());

    for i in a..=b {
        let mut dist: isize = 0;
        for j in 0..n {
            dist += sum((crabs[j] - i).abs());
        }
        ans = std::cmp::min(ans, dist);
    }

    return ans;
}

fn main() {
    let mut crabs: Vec<isize> = include_str!("../data/day7.in")
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    let ans = brute_force(&crabs);

    println!("ans = {}", ans);
}
