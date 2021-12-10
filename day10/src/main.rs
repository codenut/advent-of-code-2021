fn get_value(c: char) -> usize {
    return 0;
    // return match c {
    //     ')' => 3,
    //     ']' => 57,
    //     '}' => 1197,
    //     '>' => 25137,
    //     _ => 0
    // }
}

fn get_open(c: char) -> char {
    return match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        '>' => '<',
        _ => ' '
    };
}
fn get_part_2_value(c: char) -> usize {
    return match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    };
}

fn get_part_2_score(v: &Vec<char>) -> usize {
    let mut ans = 0;
    for c in v.iter().rev() {
        ans = ans * 5 + get_part_2_value(*c);
    }
    return ans;
}

fn get_score(line: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '[' | '{' | '(' | '<' => stack.push(c),
            ']' | '}' | ')' | '>' => {
                if stack.len() == 0 {
                    return get_value(c);
                } else {
                    let top = stack.pop().unwrap();
                    if top != get_open(c) {
                        return get_value(c);
                    }
                }
            }
            _ => println!("not gonna happen")
        }
    }

    return get_part_2_score(&stack);
}

fn main() {
    let lines: Vec<&str> = include_str!("../data/day10.in")
        .lines()
        .collect();

    let mut scores: Vec<usize> = lines.iter()
        .map(|line| get_score(line))
        .filter(|score| *score > 0)
        .collect();

    scores.sort();

    let n = scores.len();

    println!("ans={}", scores[n / 2]);
}
