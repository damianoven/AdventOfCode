use std::fs;
use regex::Regex;

fn main() {
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
    let single_digit_re: Regex = Regex::new("[0-9]{1}").unwrap();

    // iterate over each line
    for line in lines {
        // find first number in this line
        let digit_hits: Vec<&str> = single_digit_re.find_iter(line)
            .map(|m| m.as_str()).collect();
        // create two digit number
        let two_digit_number: Vec<_> = vec![digit_hits[0], digit_hits[digit_hits.len()-1]];
        let two_digit_number: String = two_digit_number.concat();
        let two_digit_number: u32 = two_digit_number.parse::<u32>().unwrap();
        // increment counter
        counter += two_digit_number;
        println!("Int. result: {}", two_digit_number);
    }

    // display result
    println!("Result: {}\n", counter);

}
