use std::fs;
use geo::{Polygon, LineString, Contains, point};

static TEST_INDICES: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1,0], [0,-1]];
static BACK_POINTERS:[[f64; 2]; 4] = [[0.0, 2.0], [-2.0, 0.0], [0.0, -2.0], [2.0, 0.0]];

fn main() {
    // load input
    let mut input: Vec<String> = load_input("input.txt");
    let start: [usize; 2] = find_start(&input);
    // decode the starting pipe
    let init_pipe: Pipe = decode_start_pipe(&input, start);
    input[start[0]] = input[start[0]].replace("S", init_pipe.get_char().to_string().as_str());
    // build a vector of indices of the loop
    let mut loop_indices: Vec<[usize; 2]> = vec![];
    loop_indices.push(start);
    loop {
        // get the current index
        let current_idx: [usize; 2] = loop_indices[loop_indices.len()-1];
        // make a pipe for it
        let current_pipe: Pipe = Pipe::new(input[current_idx[0]].chars().nth(current_idx[1]).unwrap());
        // list possible next indices
        let next_idx: Vec<[usize; 2]> = TEST_INDICES
            .to_vec()
            .iter()
            .map(|x| [
                (current_idx[0] as isize + x[0]) as usize, 
                (current_idx[1] as isize + x[1]) as usize])
            .collect();
        // list pipes from these indices
        let next_pipes: Vec<Pipe> = next_idx
            .iter()
            .map(|&x| Pipe::new(get_char_from_map(&input, x)))
            .collect();
        // find what directions are available (including backwards)
        let rotate_idx: Vec<usize> = (0..4)
            .filter(|&x| next_pipes[x]
                .pipe_is_connected(current_pipe, BACK_POINTERS[x]))
            .collect();
        // filter out hits that are already in the loop vector
        let valid_idx: Vec<usize> = rotate_idx
            .into_iter()
            .filter(|&x| !loop_indices.contains(&next_idx[x]))
            .collect();
        // check whether we reached the end
        if valid_idx.len() == 0 {
            break;
        } else {
            loop_indices.push(next_idx[valid_idx[0]]);
        }
    }
    // part 1
    println!("Part 1: {}", loop_indices.len()/2);
    // part 2
    // get the pipe coordinates
    let center_coords: Vec<[f64; 2]> = loop_indices
        .iter()
        .map(|&x| get_center_coord_from_idx(x))
        .collect();
    // create polygon
    let pipe_polygon: Polygon = Polygon::new(LineString::from(center_coords), vec![]);
    let num_inside: usize = (0..input.len())
        .map(|x| input[x]
            .char_indices()
            .filter(|y| pipe_polygon.contains(
                &point!(x: (y.0 as f64)*2.0, y: (x as f64)*(-2.0))))
            .count())
        .sum();
    println!("Part 2: {}", num_inside);
}

fn get_center_coord_from_idx(idx: [usize; 2]) -> [f64; 2] {
    [(idx[1] as f64)*2.0, (idx[0] as f64)*(-2.0)]
}


#[derive(Copy, Clone, PartialEq)]
struct Pipe {
    is_empty: bool,
    angles: [f64; 2],
}

impl Pipe {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self {is_empty: true,  angles: [ 45.0,  45.0]},
            'L' => Self {is_empty: false, angles: [  0.0,  90.0]},
            '-' => Self {is_empty: false, angles: [  0.0, 180.0]},
            'F' => Self {is_empty: false, angles: [  0.0, 270.0]},
            'J' => Self {is_empty: false, angles: [ 90.0, 180.0]},
            '|' => Self {is_empty: false, angles: [ 90.0, 270.0]},
            '7' => Self {is_empty: false, angles: [180.0, 270.0]},
            _ => panic!("match error"),
        }
    }

    fn get_char(self) -> char {
        let possible_chars: &str = ".L-FJ|7";
        let match_chars: Vec<char> = possible_chars
            .chars()
            .filter(|&x| Pipe::new(x) == self)
            .collect();
        match_chars[0]
    }

    fn pipe_is_connected(self, external_pipe: Pipe, external_center: [f64; 2]) -> bool {
        if self.is_empty || external_pipe.is_empty {
            return false;
        }
        // create segments from the external pipe and see if any of them are connected
        let external_segments: Vec<[f64; 2]> = external_pipe.angles
            .iter()
            .map(|x| [x.to_radians().cos(), x.to_radians().sin()])
            .collect();
        external_segments
            .iter()
            .any(|&x| self.segment_is_connected(x, external_center))
    }

    fn segment_is_connected(self, external_segment: [f64; 2], external_center: [f64; 2]) -> bool {
        // calculate external segment ending location
        let seg_end: [f64; 2] = [external_center[0]+external_segment[0],
            external_center[1]+external_segment[1]];
        // check if the ending location of this pipe's arms matches the external ending location
        self.angles
            .iter()
            .map(|x| [x.to_radians().cos(), x.to_radians().sin()])
            .any(|y| y[0].round() == seg_end[0].round() && y[1].round() == seg_end[1].round())
    }
}

fn get_char_from_map(full_input: &Vec<String>, idx: [usize; 2]) -> char {
    if idx[0] >= full_input.len() || idx[1] >= full_input[idx[0]].len() {
        '.'
    } else {
        full_input[idx[0]].chars().nth(idx[1]).unwrap()
    }
}

fn decode_start_pipe(full_input: &Vec<String>, start_idx: [usize; 2]) -> Pipe {
    // create an array of pipes surrounding the start
    let pipes: Vec<Pipe> = TEST_INDICES
        .iter()
        .map(|x| Pipe::new(
            get_char_from_map(full_input,
            [(start_idx[0] as isize + x[0]) as usize,
            (start_idx[1] as isize + x[1]) as usize])))
        .collect();
    // possible angles for the starting pipe
    let possible_angles: [f64; 4] = [0.0, 90.0, 180.0, 270.0];
    // filter to angles that are connected
    let actual_angles: Vec<f64> = possible_angles
        .iter()
        .filter(|&y| (0..4)
            .any(|a| pipes[a].segment_is_connected(
                [y.to_radians().cos(), y.to_radians().sin()],
                BACK_POINTERS[a])))
        .map(|y| y.to_owned())
        .collect();
    Pipe {
        is_empty: false,
        angles: actual_angles.try_into().unwrap(),
    }
}

fn find_start(full_input: &Vec<String>) -> [usize; 2] {
    let idx_one: usize = full_input.iter().position(|x| x.contains("S")).unwrap();
    let idx_two: usize = full_input[idx_one].chars().position(|x| x == 'S').unwrap();
    [idx_one, idx_two]
}

fn load_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .split("\r\n")
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}
