use std::fs;
use regex::Regex;
use itertools::any;
use std::convert::From;

fn main() {
    // create regex
    let symbol_re = Regex::new("[^.0-9]{1}").unwrap();
    let gear_re = Regex::new(r"[\*]{1}").unwrap();
    let number_re = Regex::new("[0-9]*").unwrap();

    // load input
    let lines: Vec<String> = load_input("input.txt");

    // get length of each line
    let line_length: usize = lines[0].len();

    // combine all lines into one line
    let mut super_line: String = String::from("");
    for line in lines {
        super_line = format!("{super_line}{line}");
    }

    ////////////////////////////
    // Part 1
    ////////////////////////////
    
    // get indices of symbols
    let symbol_indices: Vec<bool> = super_line.chars()
        .map(|x| symbol_re.is_match(x.to_string().as_str()))
        .collect();

    // get indices adjacent to symbols
    let symbol_adj: Vec<bool> = (0..symbol_indices.len()).map(|x|
        any(adj_indices(x.try_into().unwrap(),
        line_length.try_into().unwrap(),
        super_line.len().try_into().unwrap()),|y| symbol_indices[y.unsigned_abs()])
        ).collect();

    // get locations of numbers (start and stop indices)
    let number_sum: Vec<usize> = number_re.find_iter(&super_line)
        .map(|x| if !x.is_empty() && any(x.range(), |y| symbol_adj[y])
        {super_line[x.range()].parse::<usize>().unwrap()} else {0}).collect();

    println!("Part 1 sum: {}", number_sum.iter().sum::<usize>());

    ////////////////////////////
    // Part 2
    ////////////////////////////

    // indices of gears
    let gear_indices: Vec<usize> = (0..super_line.len())
        .filter(|&x| gear_re.is_match(super_line.chars()
        .nth(x).unwrap().to_string().as_str())).collect();

    // get numbers adjacent to gears
    let gear_nums: Vec<Vec<usize>> = gear_indices.iter().map(|&x|
            number_re.find_iter(&super_line).filter(|r|
                adj_indices(x.try_into().unwrap(),
                    line_length.try_into().unwrap(),
                    super_line.len().try_into().unwrap())
                .iter().any(|&p| r.range().contains(&p.unsigned_abs())))
                .map(|v| v.as_str().parse::<usize>().unwrap())
                .collect()).collect();
    
    // sum numbers if there are exactly two
    let part_two_sum: usize = gear_nums.iter().map(|x|
        if x.len() == 2 {x.iter().product()} else {0}).sum();
    println!("Part 2 sum: {}", part_two_sum);
    
}

fn load_input(file_name: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_name).unwrap();
    let newline: &str = "\r\n";
    contents.strip_suffix(newline).unwrap()
        .split(newline)
        .map(|x| x.to_string())
        .collect()
}

fn adj_indices(index: isize, line_length: isize, line_len: isize) -> Vec<isize> {
    let mut a = vec![
        index-line_length-1, index-line_length, index-line_length+1,
        index - 1, index, index + 1,
        index+line_length-1, index+line_length, index+line_length+1];
    a.retain(|&x| x >= 0 && x < line_len);
    a
}