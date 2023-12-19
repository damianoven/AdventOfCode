use std::fs;

fn main() {
    // load input
    let part1: bool = false;
    let input = load_input("input.txt", part1);
    // Part 1, 2
    let mut num_total: usize = 0;
    for (i, entry) in input.iter().enumerate() {
        let mut valid_cases: usize = 0;
        gen_cases(&mut valid_cases, &entry.0, &entry.1);
        num_total += valid_cases;
        println!("[Case {:04}] Valid: {}", i, valid_cases);
    }
    println!("Part 1: {}", num_total);
}

fn gen_cases(valid_cases: &mut usize, current_case: &String, group_sizes: &Vec<usize>) {
    // check if this case is impossible
    if is_impossible(current_case, group_sizes) {
        // do nothing and return... this branch is dead
        return;
    }
    // check if this is the base case
    if !current_case.contains("?") {
        // base case!
        // increment the number of cases
        *valid_cases += 1;
        println!("{}", valid_cases);
        return;
    }
    // recurse assuming the first '?' is either '.' or '#'
    gen_cases(valid_cases,
        &current_case.replacen("?", ".", 1),
        group_sizes);
    gen_cases(valid_cases,
        &current_case.replacen("?", "#", 1),
        group_sizes);
    return;
}

fn is_impossible(case: &String, group_sizes: &Vec<usize>) -> bool {
    // returns T/F for if the given list and group sizes are incompatible.

    // append '.' to the list of chars
    let mut case_chars: Vec<char> = case.chars().collect();
    case_chars.push('.');
    // try to fit in each group
    let mut idx_search: usize = 0;
    for i in 0..group_sizes.len() {
        // try each index
        let idx_start = (idx_search..case.len()-group_sizes[i]+1)
            .find(|&f| (0..group_sizes[i]+1)
                .all(|x| if x < group_sizes[i] {
                    case_chars[f+x]=='?' || case_chars[f+x]=='#'
                } else {
                    case_chars[f+x]=='?' || case_chars[f+x]=='.'
                }));
        if idx_start.is_none() {
            // could not find a place for this group
            return true;
        } else {
            // "insert" this group
            idx_search = idx_start.unwrap() + group_sizes[i] - 1 + 1;
        }
    }
    // check that the number of '#' matches the group sum
    if !case.contains("?") && (case.chars().filter(|&x| x=='#').count() != group_sizes.iter().sum()) {
        return true;
    }
    return false;
}

fn load_input(file_name: &str, part1: bool) -> Vec<(String, Vec<usize>)> {
    let input: String = fs::read_to_string(file_name).unwrap();
    let mut input: Vec<(String, Vec<usize>)> = input
        .split("\r\n")
        .filter(|x| !x.is_empty())
        .map(|x| (
            x.split_whitespace()
                .nth(0)
                .unwrap()
                .to_string(),
            x.split_whitespace()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .repeat(if part1 {1} else {5})))
        .collect();
    if !part1 {
        for i in 0..input.len() {
            let mut temp_string: String = input[i].0.clone();
            temp_string.push_str("?");
            temp_string = temp_string.repeat(5);
            temp_string.remove(temp_string.len()-1);
            input[i].0 = temp_string;
        }
    }
    input
}