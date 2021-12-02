fn prob1() {
    let items: Vec<usize> = include_str!("../data/day1.in")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let mut ans = 0;
    for i in 1..items.len() {
        if items[i] > items[i - 1] {
            ans += 1;
        }
    }

    println!("{}", ans)
}

fn prob2() {
    let nums: Vec<usize> = include_str!("../data/day1.in")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let psum: usize = nums[0 .. 3].iter().sum();
    let mut ans = 0;

    for i in 3..nums.len() {
        let nsum = psum + nums[i] - nums[i - 3];
        if nsum > psum {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn main() {
    prob1();
    prob2();
}
