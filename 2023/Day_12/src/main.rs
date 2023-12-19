use std::fs;
use regex::Regex;

fn main() {
    // load input
    let part1: bool = false;
    let input = load_input("input.txt", part1);
    // Part 1, 2
    let mut num_total: usize = 0;
    for (i, entry) in input.iter().enumerate() {
        // initialize valid case counter
        let mut valid_cases: usize = 0;
        // create regex for this group pattern
        let group_re = build_regex(&entry.1);
        gen_cases(&mut valid_cases, &entry.0, &entry.1, &group_re);
        num_total += valid_cases;
        println!("[Case {:04}] Valid: {}", i, valid_cases);
    }
    println!("Part 1: {}", num_total);
}

fn build_regex(group_sizes: &Vec<usize>) -> Regex {
    let group_matches: Vec<String> = group_sizes.iter()
        .map(|&x| format!("([#|?]{{{}}}[O|?]{{1}})", x))
        .collect();
    let misc_match: String = "([O?]*)".to_string();
    let full_match = "(^[O?]*)".to_owned() + &group_matches.join(&misc_match);
    Regex::new(&full_match).unwrap()
}

fn gen_cases(valid_cases: &mut usize, current_case: &String, group_sizes: &Vec<usize>, group_re: &Regex) {
    // check if this case is impossible
    if !group_re.is_match(&current_case) {
        // this does not work
        return;
    }
    // check if this is the base case
    if !current_case.contains("?") {
        // base case!
        // check that this is a valid solution
        if current_case.chars().filter(|&x| x=='#').count() == group_sizes.iter().sum() {
            // increment the number of cases
            *valid_cases += 1;
        }
        return;
    }
    // recurse assuming the first '?' is either '.' or '#'
    gen_cases(valid_cases,
        &current_case.replacen("?", "O", 1),
        group_sizes, group_re);
    gen_cases(valid_cases,
        &current_case.replacen("?", "#", 1),
        group_sizes, group_re);
    return;
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
                .to_string()
                .replace('.', "O"),
            x.split_whitespace()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .repeat(if part1 {1} else {5})))
        .collect();
        for i in 0..input.len() {
            if part1 {
                input[i].0 = input[i].0.clone() + "O";
            } else {
                let mut temp_string: String = input[i].0.clone();
                temp_string.push_str("?");
                temp_string = temp_string.repeat(5);
                temp_string.remove(temp_string.len()-1);
                input[i].0 = temp_string + "O";
            }
        }
    input
}