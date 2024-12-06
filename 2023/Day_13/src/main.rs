use std::fs;

fn main() {
    let input = load_input("input.txt");
    let mut part_one_counter: usize = 0;
    for puzzle in input {
        let mirror = find_mirror(&puzzle);
        part_one_counter += (if mirror.1 {100} else {1}) * (mirror.0 + 1);
    }
    println!("Part one: {}", part_one_counter);
}

fn find_mirror(input: &Vec<Vec<char>>) -> (usize, bool) {
    // check rows first
    for i_row in 0..input.len()-1 {
        let mut all_mirror: bool = true; // assume true, then check for false
        let mut fan: usize = 0;
        loop {
            // check if equal
            if !check_equal(input, ((i_row - fan) as isize, (i_row + 1 + fan) as isize), true) {
                // mismatch
                all_mirror = false;
                break;
            }
            // go to the next fan
            fan += 1;
            if i_row < fan || i_row + 1 + fan >= input.len() {
                break;
            }
        }
        if all_mirror {
            return (i_row, true);
        }
    }
    // now check cols
    for i_col in 0..input[0].len()-1 {
        let mut all_mirror: bool = true;
        let mut fan: usize = 0;
        loop {
            if !check_equal(input, ((i_col - fan) as isize, (i_col + 1 + fan) as isize), false) {
                all_mirror = false;
                break;
            }
            fan += 1;
            if i_col < fan || i_col + 1 + fan >= input[0].len() {
                break;
            }
        }
        if all_mirror {
            return (i_col, false);
        }
    }
    panic!("Error");
}

fn check_equal(input: &Vec<Vec<char>>, idx: (isize, isize), is_row: bool) -> bool {
    // input validation
    if idx.0 < 0 || idx.1 < 0 {
        return false;
    }
    if is_row {
        if idx.0 as usize >= input.len() || idx.1 as usize >= input.len() {
            return false;
        }
    } else {
        if idx.0 as usize >= input[0].len() || idx.1 as usize >= input[0].len() {
            return false;
        }
    }
    if is_row {
        return (0..input[0].len())
            .all(|x| input[idx.0 as usize][x] == input[idx.1 as usize][x]);
    } else {
        return (0..input.len())
            .all(|x| input[x][idx.0 as usize] == input[x][idx.1 as usize]);
    }
}

fn load_input(file_name: &str) -> Vec<Vec<Vec<char>>> {
    let text = fs::read_to_string(file_name).unwrap();
    text
        .split("\r\n\r\n")
        .filter(|x| !x.is_empty())
        .map(|x| x
            .split("\r\n")
            .filter(|y| !y.is_empty())
            .map(|z| z.chars().collect::<Vec<char>>())
            .collect())
        .collect()
}