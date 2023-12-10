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
    let mut current_min: usize = usize::MAX;
    (0..seeds.len()/2).for_each(|x| (seeds[x*2]..seeds[x*2]+seeds[x*2+1])
        .for_each(|y| if do_all_maps(&maps, y) < current_min {current_min = do_all_maps(&maps, y)}));
    println!("Part 2: {}", current_min);
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