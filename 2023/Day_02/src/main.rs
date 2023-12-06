use std::fs;
use regex::Regex;
use itertools::Itertools;
use itertools::all;

fn main() {
    // constants
    let colors = ["red", "green", "blue"];
    let max_quantities = [12, 13, 14];

    // initialize counters
    let mut counter_1: usize= 0;
    let mut counter_2: usize = 0;

    // create regex
    let num_re: Regex = Regex::new("[0-9]+").unwrap();
    let pull_re = colors.map(|x| Regex::new(format!("[0-9]+ {x}").as_str()).unwrap());
    
    // load input
    let lines: Vec<String> = load_input("input.txt");

    // go through each line
    for (idx, line) in lines.iter().enumerate() {
        // get the max value for each color on this line
        let max_pulls: Vec<usize> = pull_re.iter()
            .map(|x| x.find_iter(line)
            .map(|y|num_re.find(y.as_str()).unwrap()
            .as_str().parse::<usize>().unwrap()).max_set())
            .map(|z| z[0]).collect();
        // part one
        let game_valid = all(0..colors.len(),
                |a| max_pulls[a]<=max_quantities[a]);
        counter_1 += if game_valid {idx+1} else {0};
        // part two
        let game_power: usize = max_pulls.iter().product();
        counter_2 += game_power;
    }
    println!("Part 1 counter: {counter_1}");
    println!("Part 2 counter: {counter_2}");
}

fn load_input(file_name: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_name).unwrap();
    let newline: &str = "\r\n";
    contents.strip_suffix(newline).unwrap()
        .split(newline)
        .map(|x| x.to_string())
        .collect()
}