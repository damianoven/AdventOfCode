use std::fs;

fn main() {
    // load input
    let input: Vec<Vec<char>> = load_file("input.txt");
    // expand along dimension one
    let idx_blank1: Vec<usize> = (0..input.len())
        .filter(|&x| input[x].iter().all(|&y| y=='.'))
        .collect();
    // expand along dimension two
    let idx_blank2: Vec<usize> = (0..input[0].len())
        .filter(|&x| (0..input.len()).all(|y| input[y][x]=='.'))
        .collect();
    // find galaxies
    let galaxies: Vec<[isize; 2]> = (0..input.len())
        .map(|x| (0..input[x].len()).map(|y| [x as isize, y as isize])
            .filter(|z| input[z[0] as usize][z[1] as usize]=='#')
            .collect::<Vec<[isize; 2]>>())
        .flatten()
        .collect();
    // compute distance between galaxies
    let num_insert: usize = 1000000-1;
    let mut distances: Vec<[usize; 3]> = vec![]; // [idx1, idx2, distance]
    for a in 0..galaxies.len() {
        for b in a+1..galaxies.len() {
            let distance_y: usize = (galaxies[b][0] - galaxies[a][0]).abs() as usize +
                num_insert * idx_blank1
                    .iter()
                    .filter(|&&f| f < std::cmp::max(galaxies[a][0] as usize, galaxies[b][0] as usize)
                        && f > std::cmp::min(galaxies[a][0] as usize, galaxies[b][0] as usize))
                    .count();
            let distance_x: usize = (galaxies[b][1] - galaxies[a][1]).abs() as usize +
                num_insert * idx_blank2
                    .iter()
                    .filter(|&&f| f < std::cmp::max(galaxies[a][1] as usize, galaxies[b][1] as usize)
                        && f > std::cmp::min(galaxies[a][1] as usize, galaxies[b][1] as usize))
                    .count();
            distances.push([a, b, distance_x + distance_y]);
        }
    }
    println!("Distance: {}", distances.iter().map(|x| x[2]).sum::<usize>())
}

fn load_file(file_name: &str) -> Vec<Vec<char>> {
    let input: String = fs::read_to_string(file_name).unwrap();
    input
        .split("\r\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect()
}

fn print_data(data: Vec<Vec<char>>) {
    data.iter().for_each(|f| println!("{:?}", f));
}
