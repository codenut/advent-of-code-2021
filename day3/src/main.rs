fn get0(lines: &Vec<&str>) -> usize {
    let ans = 0;
    let m = lines[0].len();
    let mut nlines = lines.to_vec();

    for i in 0..m {
        let mut o: Vec<&str> = Vec::new();
        let mut z: Vec<&str> = Vec::new();
        for line in &nlines {
            let c = line.chars().nth(i).unwrap();
            if c == '0' {
                z.push(line)
            } else {
                o.push(line);
            }
        }

        if z.len() <= o.len() {
            nlines = z.to_vec();
        } else {
            nlines = o.to_vec();
        }

        if nlines.len() == 1 {
            return usize::from_str_radix(nlines[0], 2).unwrap();
        }
    }

    return 0;
}

fn get1(lines: &Vec<&str>) -> usize {
    let ans = 0;
    let m = lines[0].len();
    let mut nlines = lines.to_vec();

    for i in 0..m {
        let mut o: Vec<&str> = Vec::new();
        let mut z: Vec<&str> = Vec::new();
        for line in &nlines {
            let c = line.chars().nth(i).unwrap();
            if c == '0' {
                z.push(line)
            } else {
                o.push(line);
            }
        }

        if o.len() >= z.len() {
            nlines = o.to_vec();
        } else {
            nlines = z.to_vec();
        }

        if nlines.len() == 1 {
            return usize::from_str_radix(nlines[0], 2).unwrap();
        }
    }

    return 0;
}

fn prob2() {
    let lines: Vec<&str> = include_str!("../data/day3.in")
        .lines()
        .collect();
    let a = get1(&lines);
    let b = get0(&lines);

    println!("{} * {} = {}", a, b, a * b)
}

fn prob1() {
    let lines: Vec<&str> = include_str!("../data/day3.in")
        .lines()
        .collect();

    let m = lines[0].len();
    let mut alpha = 0;
    let mut eps = 0;

    for i in (0..m).rev() {
        let mut z = 0;
        let mut o = 0;
        for line in &lines {
            let c = line.chars().nth(i).unwrap();
            if c == '0' {
                z += 1;
            } else {
                o += 1;
            }
        }

        let k = m - i - 1;
        if o > z {
            alpha += 1 << k;
        } else {
            eps += 1 << k;
        }
    }

    println!("{} * {} = {}", alpha, eps, alpha * eps);
}

fn main() {
    // prob1();
    prob2();
}
