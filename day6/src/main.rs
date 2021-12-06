fn simulate_arr(counters: [usize; 9]) -> [usize; 9] {
    let mut new_counter: [usize; 9] = [0; 9];

    for i in 0..counters.len() {
        let count = counters[i];
        if i == 0 {
            new_counter[6] += count;
            new_counter[8] += count;
        } else {
            new_counter[i - 1] += count;
        }
    }

    return new_counter;
}

fn main() {
    let fishes: Vec<usize> = include_str!("../data/day6.in")
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    let mut counter_array: [usize; 9] = [0; 9];

    for fish in fishes {
        counter_array[fish] += 1;
    }

    let days = 256;
    for _ in 1..=days {
        counter_array = simulate_arr(counter_array);
    }
    let sum: usize = counter_array.iter().sum();

    println!("size after {} days = {}", days, sum);
}
