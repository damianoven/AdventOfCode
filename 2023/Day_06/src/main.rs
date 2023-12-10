use std::fs;

fn main() {
    // load data
    let data: Vec<Vec<usize>> = load_input("input2.txt");
    // part one
    let possible_distances: Vec<Vec<usize>> = data[0].iter()
        .map(|&x| (0..=x).map(|y| (x-y)*y).collect()).collect();
    let num_winning: Vec<usize> = (0..data[0].len())
        .map(|x| possible_distances[x].iter()
            .filter(|&&y| y > data[1][x]).count()).collect();
    println!("Part 1: {}", num_winning.iter().product::<usize>());
}

fn load_input(file_name: &str) -> Vec<Vec<usize>> {
    // load to string
    let contents: String = fs::read_to_string(file_name).unwrap();
    // split on newline
    let out_vec: Vec<Vec<usize>> = contents
        .split("\r\n")
        .filter(|&x| !x.is_empty())
        .map(|y| y[y.find(':').unwrap()+1..y.len()].to_string())
        .map(|z|z.split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect())
        .collect();
    out_vec
}
