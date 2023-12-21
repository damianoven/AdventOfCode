use std::{fs, collections::hash_map};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // load input
    let part1: bool = false;
    let input = load_input("input.txt", part1);
    // Part 1, 2
    let mut num_total: usize = 0;
    for (i, entry) in input.iter().enumerate() {
        let mut cache: HashMap<String, usize> = HashMap::new();
        num_total += num_valid(&entry.0, 0, entry.1.clone(), &mut cache);
        println!("[Case {:04}] done", i);
    }
    println!("Part 1: {}", num_total);
}

fn num_valid(case: &String, start_idx: usize, group_sizes: Vec<usize>, solution_cache: &mut HashMap<String, usize>) -> usize {
    // constants
    let hash: String = "#".to_string();
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
        // min number of entries to satisfy groups
        let min_len: usize = group_sizes.iter().sum::<usize>() + group_sizes.len();
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
                let hash_key: String = format!("{}{}{}_#$k{}{}{}",
                case, i+group_sizes[0]+1, group_sizes.len()-1, i+group_sizes[0]+1, case, group_sizes.len()-1);
                let mut remaining_valid: usize = 0;
                if solution_cache.contains_key(&hash_key) {
                    // we've computed this before... just grab the result
                    remaining_valid = *solution_cache.get(&hash_key).unwrap();
                } else {
                    // need to compute this for the first time
                    remaining_valid = num_valid(case, i+group_sizes[0]+1,
                        group_sizes[1..].to_vec(), solution_cache);
                    // add to map
                    solution_cache.insert(hash_key, remaining_valid);
                }
                running_total += remaining_valid;
            }
        }
        return running_total;
    }
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