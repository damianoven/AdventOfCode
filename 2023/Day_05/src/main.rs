use std::fs;
use regex::Regex;

fn main() {
    // constants
    let map_names = ["seed-to-soil map",
        "soil-to-fertilizer map", "fertilizer-to-water map",
        "water-to-light map", "light-to-temperature map",
        "temperature-to-humidity map", "humidity-to-location map"];
    // load seeds
    let seeds: Vec<usize> = load_group("input.txt", "seeds");
    // load maps
    let maps: Vec<Vec<usize>> = map_names.iter().
        map(|&x| load_group("input.txt", x)).collect();
    // part one
    let part_one_vals: Vec<usize> = seeds.iter().map(|&x| do_all_maps(&maps, x)).collect();
    println!("Part 1: {}", part_one_vals.iter().min().unwrap());
    // part two
    let part_two_min: usize = (0..seeds.len()/2).map(|x|
            do_all_maps_range(&maps, (seeds[x*2], seeds[x*2]+seeds[x*2+1]))
            .iter().map(|&y| y.0).min().unwrap()).min().unwrap();
    println!("Part 2: {}", part_two_min);
}

fn do_all_maps_range(maps: &Vec<Vec<usize>>, in_range: (usize, usize)) -> Vec<(usize, usize)> {
    let mut out_val: Vec<(usize, usize)> = vec![in_range.clone()];
    for map in maps {
        out_val = out_val.iter().map(|&x| map_range(map, x)).flatten().collect();
    }
    out_val
}

// take a range (rhs exclusive) and returns one or more mapped output ranges
fn map_range(in_map: &Vec<usize>, in_range: (usize, usize)) -> Vec<(usize, usize)> {
    // copy input range as mut
    let mut current_range: [usize; 2] = [in_range.0, in_range.1];
    // initialize output vec
    let mut out_vec: Vec<(usize, usize)> = Vec::new();
    // create vec of map entries that apply here
    let mut map_entries: Vec<(usize, usize, usize)> = (0..in_map.len()/3)
        .map(|x| (in_map[x*3], in_map[x*3+1], in_map[x*3+2]))
        .filter(|&y| y.1 < in_range.1 && y.1+y.2 > in_range.0).collect();
    // sort by ascending start index
    map_entries.sort_by(|&a, &b| a.1.cmp(&b.1));
    // build output ranges
    for map_range in map_entries {
        // check if we need to trim to this range start
        if map_range.1 > current_range[0] {
            // add this pre-range to the output
            out_vec.push((current_range[0], map_range.1-1));
            current_range[0] = map_range.1;
        }
        // get the range start and stop
        let range_start: usize = std::cmp::max(map_range.1, current_range[0]);
        let range_stop: usize = std::cmp::min(map_range.1+map_range.2, current_range[1]);
        let out_start: usize = map_range.0 + (range_start-map_range.1);
        let out_end: usize = out_start + (range_stop-range_start);
        // add this range
        out_vec.push((out_start, out_end));
        current_range[0] = range_stop;
    }
    // check if anything is remaining
    if current_range[0] < current_range[1] {
        out_vec.push((current_range[0], current_range[1]));
        current_range[0] = current_range[1];
    }
    out_vec // return
}

fn do_all_maps(maps: &Vec<Vec<usize>>, val:usize) -> usize {
    // apply all maps successively to this value
    let mut temp_val = val;
    for map in maps {
        temp_val = map_value(&map, temp_val);
    }
    temp_val
}

fn map_value(map: &Vec<usize>, val: usize) -> usize {
    // iterate through each weird segment
    let seg_match: Vec<usize> = (0..map.len()/3)
        .filter(|x| val >= map[x*3+1] && val < map[x*3+1] + map[x*3+2]).collect();
    if seg_match.len() == 0 {
        // just return the input value
        val
    } else {
        // return the mapped value
        let range_loc: usize = val - map[seg_match[0]*3+1];
        map[seg_match[0]*3] + range_loc
    }
}

fn load_group(file_name: &str, group_name: &str) -> Vec<usize> {
    // load file to big string
    let contents: String = fs::read_to_string(file_name).unwrap();
    // create regex for this group
    let group_re = Regex::new(format!("{}:[\n\r0-9 ]*", group_name).as_str()).unwrap();
    // get the regex match
    let group_match = group_re.find(&contents).unwrap().as_str();
    // clean it up
    let group_content: String = group_match[group_match.find(':').unwrap()+1..group_match.len()].to_string();
    let group_content: String = group_content.replace("\r\n", " ");
    // convert to numbers
    group_content.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}
