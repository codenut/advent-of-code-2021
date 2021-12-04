use std::collections::HashMap;


struct Card {
    nums: HashMap<usize, (usize, usize)>,
    rows: [u8; 5],
    cols: [u8; 5],
    board: String,
    won: bool
}

fn main() {
    let lines: Vec<&str> = include_str!("../data/day4.in")
        .lines()
        .collect();

    let mut cards: Vec<Card> = Vec::new();

    for i in (1..lines.len()).step_by(6) {
        let mut nums: HashMap<usize, (usize, usize)> = HashMap::new();

        let mut strs: Vec<&str> = Vec::new();

        for row in 0..5 {
            let k = i + row + 1;

            let cnums: HashMap<usize, (usize, usize)> = lines[k]
                .split_whitespace()
                .enumerate()
                .map(|(col, i)| (i.parse().unwrap(), (row, col)))
                .collect();

            nums.extend(&cnums);

            strs.push(lines[k]);
        }

        let card: Card = Card {
            nums: nums,
            cols: [0; 5],
            rows: [0; 5],
            board: strs.join("\n"),
            won: false
        };

        cards.push(card);
    }

    let draws: Vec<usize> = lines[0]
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    for (_, num) in draws.iter().enumerate() {
        for card in &mut cards {
            let hasnum = card.nums.contains_key(&num);

            if hasnum {
                let (row, col) = (card.nums.get(&num)).unwrap();

                card.rows[*row] += 1;
                card.cols[*col] += 1;

                if !card.won && (card.rows[*row] == 5 || card.cols[*col] == 5) {
                    let mut sum: usize = card.nums.keys().sum();
                    sum -= num;

                    println!("{} * {} = {}", sum, num, sum * num);
                    println!("board = {}", card.board);
                    // return; part1 asks for the first board to win;
                    card.won = true
                }

                card.nums.remove(&num);
            }
        }
    }

}
