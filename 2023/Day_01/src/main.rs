use std::fs;
use regex::Regex;

fn main() {
    // choose solution for part 1 or part 2
    let part1: bool = false; // part 2

    // Read input data
    let contents: String = fs::read_to_string("input.txt")
        .expect("Error");

    // split on newline
    let newline: &str = "\r\n";
    let lines: Vec<&str> = contents
        .strip_suffix(newline).expect("Strip error")
        .split(newline).collect();

    // initialize counter
    let mut counter: u32 = 0;

    // initialize regular expressions
    let single_digit_re: Regex = Regex::new("^[0-9]{1}").unwrap();
    let part_two_matches: Vec<_> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ""];
    let mut part_two_re_text: String = part_two_matches.join("{1}|^")
        .strip_suffix("|^").unwrap().to_string();
    part_two_re_text.insert(0, '^');
    let complex_re: Regex = Regex::new(&part_two_re_text).unwrap();
    let re: Regex = if part1 {single_digit_re} else {complex_re};

    // iterate over each line
    for line in lines {
        // find hits in this line
        // search each substring of the line starting with the full line
        // we have to do it this way to find overlapping pattern matches
        let digit_hits: Vec<&str> = line.char_indices()
            .filter_map(|(i, _)| re.find(&line[i..]))
            .map(|y| y.as_str()).collect();
        // get first digit
        let first_digit: String = if digit_hits[0].to_string().len() > 1 {
            // complex hit
            part_two_matches[part_two_matches.iter()
                .position(|&x| x==digit_hits[0].to_string())
                .unwrap()-9].to_string()
        } else {digit_hits[0].to_string()};
        // get second digit
        let second_digit: String = if digit_hits[digit_hits.len()-1].to_string().len() > 1 {
            // complex hit
            part_two_matches[part_two_matches.iter()
                .position(|&x| x==digit_hits[digit_hits.len()-1].to_string())
                .unwrap()-9].to_string()
        } else {digit_hits[digit_hits.len()-1].to_string()};
        // create two digit number
        let two_digit_number: u32 = [first_digit, second_digit].concat().parse::<u32>().unwrap();
        // increment counter
        counter += two_digit_number;
    }

    // display result
    println!("Result: {}\n", counter);

}
