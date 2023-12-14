use std::fs;

fn main() {
    // load input
    let input: Vec<Vec<isize>> = load_input("input.txt");
    // allocate pyramids
    let mut pyramids: Vec<Vec<Vec<isize>>> = input
        .iter()
        .map(|x| vec![x.clone()])
        .collect();
    // part 1
    for i in 0..input.len() {
        while !pyramids[i][pyramids[i].len()-1].iter().all(|&x| x == 0) {
            let diffs: Vec<isize> = pyramids[i][pyramids[i].len()-1]
                .windows(2)
                .map(|x| x[1]-x[0])
                .collect();
            pyramids[i].push(diffs);
        }
    }
    let next_vals: Vec<isize> = pyramids
        .iter()
        .map(|x| x.iter().map(|y| y[y.len()-1]).sum())
        .collect();
    println!("Part 1: {}", next_vals.iter().sum::<isize>());
    // part 2
    let prev_vals: Vec<isize> = pyramids
        .iter()
        .map(|x| x.iter()
            .enumerate()
            .map(|y| y.1[0] * (if y.0 % 2 == 0 {1} else {-1}))
            .sum::<isize>())
        .collect();
    println!("Part 2: {}", prev_vals.iter().sum::<isize>());
}

fn load_input(file_name: &str) -> Vec<Vec<isize>> {
    // load file
    let contents = fs::read_to_string(file_name).unwrap();
    // split into lines
    contents
        .split("\r\n")
        .filter(|&x| !x.is_empty())
        .map(|y| y.split_whitespace()
            .map(|z| z.parse::<isize>().unwrap())
            .collect())
        .collect()
}