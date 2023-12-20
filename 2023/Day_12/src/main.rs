use std::fs;
use regex::Regex;

fn main() {
    // load input
    let part1: bool = false;
    let input = load_input("input.txt", part1);
    // Part 1, 2
    let mut num_total: usize = 0;
    for (i, entry) in input.iter().enumerate() {
        num_total += num_valid(&entry.0, 0, entry.1.clone());
        println!("[Case {:04}] done", i);
    }
    println!("Part 1: {}", num_total);
}

fn num_valid(case: &String, start_idx: usize, group_sizes: Vec<usize>) -> usize {
    // constants
    let hash: String = "#".to_string();
    // min number of entries to satisfy groups
    let min_len: usize = group_sizes.iter().sum::<usize>() + group_sizes.len();
    // characters available
    let chars_available: usize = case.len() - start_idx;
    // if there isn't room, then return 0
    if min_len > chars_available {
        return 0;
    }
    // check if base case
    if group_sizes.len() == 1 {
        // calculate how many ways the entry can fit in the remaining characters
        return (start_idx..case.len()-group_sizes[0])
            .filter(|&x| 
                !case[start_idx..x].contains("#") &&
                !case[x+group_sizes[0]+1..].contains("#") &&
                !case[x..x+group_sizes[0]].contains("O") && 
                case[x+group_sizes[0]..x+group_sizes[0]+1]!=hash).count();
    } else {
        // adjust the first group from the first position to the last available position
        let mut running_total: usize = 0;
        for i in start_idx..case.len()-min_len+1 {
            // check if we went too far
            if case[start_idx..i].contains("#") {
                break;
            }
            // check if the first group can fit here
            if !case[i..i+group_sizes[0]].contains("O") && 
                case[i+group_sizes[0]..i+group_sizes[0]+1]!=hash && 1==1 {
                // this is valid, leave the first group here and recurse on remaining groups
                running_total += num_valid(case, i+group_sizes[0]+1, group_sizes[1..].to_vec());
            }
        }
        return running_total;
    }
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
    // check if 
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