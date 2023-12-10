use std::fs;

struct Hand {
    cards: [u8; 5],
    group: u8,
    bid: usize,
}

impl Hand {
    fn score_hand(&mut self) {
        // bin the cards
        let mut bins: [u8; 15] = [0; 15];
        self.cards.iter().for_each(|&f| bins[f as usize] += 1);
        // figure out the group
        let bins_compact: Vec<u8> = bins.iter().filter(|&&w| w > 0).map(|u| *u).collect();
        let max_bin: u8 = *bins_compact.iter().max().unwrap();
        let min_bin: u8 = *bins_compact.iter().min().unwrap();
        match max_bin {
            5 => self.group = 6, // 5 of a kind
            4 => self.group = 5, // 4 of a kind
            3 => {
                if min_bin == 2 {
                    self.group = 4 // full house
                } else {
                    self.group = 3 // 3 of a kind
                }
            }
            2 => {
                if bins_compact.len() == 3 {
                    self.group = 2 // two pair
                } else {
                    self.group = 1 // one pair
                }
            }
            1 => self.group = 0, // high card
            _ => panic!("error"),
        }
    }
}

fn main() {
    // load input
    let mut hands: Vec<Hand> = load_input("input.txt");
    // find group of each hand
    hands.iter_mut().for_each(|x| x.score_hand());
    // sort
    hands.sort_by(|a, b| {
        if a.group != b.group {
            a.group.cmp(&b.group)
        } else {
            a.cards.cmp(&b.cards)
        }
    });
    // part one
    let part_one_sum: usize = (0..hands.len()).map(|x| (x+1)*(hands[x].bid)).sum();
    println!("Part 1: {}", part_one_sum);
}

fn load_input(file_name: &str) -> Vec<Hand> {
    // load as string
    let file_contents: String = fs::read_to_string(file_name).unwrap();
    // split on newline and create a hand from each line
    file_contents
        .split("\r\n")
        .filter(|&x| !x.is_empty())
        .map(|y| create_hand(y))
        .collect()
}

fn create_hand(text_line: &str) -> Hand {
    // split into two parts
    let inputs: Vec<String> = text_line
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    // create array of cards
    let cards: [u8; 5] = inputs[0]
        .chars()
        .map(|x| interp_char(x))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    // initialize hand
    Hand {
        cards,
        group: 0 as u8,
        bid: inputs[1].parse::<usize>().unwrap(),
    }
}

fn interp_char(input: char) -> u8 {
    match input {
        'A' => 14 as u8,
        'K' => 13 as u8,
        'Q' => 12 as u8,
        'J' => 11 as u8,
        'T' => 10 as u8,
        _ => input.to_string().parse::<u8>().unwrap(),
    }
}
