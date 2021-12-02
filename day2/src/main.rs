fn prob2() {
    let lines = include_str!("../data/day2.in")
        .lines();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let cmd = parts[0];
        let cnt: usize = parts[1].parse().unwrap();

        if cmd == "forward" {
            x += cnt;
            y = y + (aim * cnt);
        } else if cmd == "down" {
            aim += cnt;
        } else {
            aim -= cnt;
        }
    }

    println!("{} * {}  = {}", x, y, x * y);
}

fn prob1() {
    let lines = include_str!("../data/day2.in")
        .lines();

    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let cmd = parts[0];
        let cnt: usize = parts[1].parse().unwrap();

        if cmd == "forward" {
            x += cnt;
        } else if cmd == "down" {
            y += cnt;
        } else {
            y -= cnt;
        }
    }

    println!("{}", x * y);
}

fn main() {
    prob1();
    prob2();
}
