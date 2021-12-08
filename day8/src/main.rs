use std::collections::HashMap;
// TODO refactor this

fn is_match(five: &String, six: &String) -> bool {
    return five.chars().all(|c| six.contains(c));
}

fn sort_string(s: &String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let t: String = String::from_iter(chars);
    return t;
}

fn process(line: &str) -> usize {
    let parts: Vec<&str> = line.split(" | ").collect();
    let pats: Vec<String> = parts[0]
        .split(" ")
        .map(|s| sort_string(&s.to_string()))
        .collect();

    let mut map: HashMap<String, usize> = HashMap::new();
    let mut fives: Vec<String> = Vec::new();
    let mut sixes: Vec<String> = Vec::new();

    for pat in pats {
        if pat.len() == 2 {
            map.insert(pat, 1);
        } else if pat.len() == 4 {
            map.insert(pat, 4);
        } else if pat.len() == 3 {
            map.insert(pat, 7);
        } else if pat.len() == 7 {
            map.insert(pat, 8);
        } else if pat.len() == 6 {
            sixes.push(pat);
        } else {
            fives.push(pat);
        }
    }

    let mut hasone = false;
    for five in &fives {
        if map.contains_key(five) {
            continue;
        }
        let mut matches: Vec<&String> = Vec::new();
        for six in &sixes {
            if map.contains_key(six) {
                continue;
            }
            if is_match(five, six) {
                matches.push(six);
            }
        }

        if matches.len() == 0 {
            map.insert(five.to_string(), 2);
        } else if matches.len() == 1 && !hasone {
            map.insert(five.to_string(), 3);
            map.insert(matches[0].to_string(), 9);
            hasone = true;
        }
    }

    for five in &fives {
        if map.contains_key(five) {
            continue;
        }
        let mut matches: Vec<&String> = Vec::new();
        for six in &sixes {
            if map.contains_key(six) {
                continue;
            }
            if is_match(&five, &six) {
                matches.push(six);
            }
        }

        if matches.len() == 1 {
            map.insert(five.to_string(), 5);
            map.insert(matches[0].to_string(), 6);
        }
    }

    for six in sixes {
        if map.contains_key(&six) {
            continue;
        }
        map.insert(six, 0);
    }

    let sigs: Vec<String> = parts[1].split(" ")
        .map(|s| sort_string(&s.to_string()))
        .collect();

    let mut digs: usize = 0;

    for sig in sigs {
        digs = digs * 10 + map.get(&sig).unwrap();
    }

    return digs;
}

fn main() {
    let lines: Vec<&str> = include_str!("../data/day8.in")
        .lines()
        .collect();

    let mut ans: usize = 0;
    for line in lines {
        ans += process(line);
    }

    println!("{}", ans);
}