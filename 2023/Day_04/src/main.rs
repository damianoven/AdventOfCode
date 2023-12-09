use std::fs;

fn main() {
    
    // load input
    let (winning_nums, playing_nums) = load_input("input.txt");
    
    // part one
    let num_win: Vec<usize> = (0..playing_nums.len())
        .map(|x| (0..playing_nums[x].len())
        .filter(|&y| winning_nums[x].contains(&playing_nums[x][y])).count()
        ).collect();
    let part_one_total: usize = num_win.iter()
        .map(|&c| if c > 0 {usize::pow(2, (c-1).try_into().unwrap())} else {0}).sum();
    println!("Part 1 sum: {}", part_one_total);

    // part two
    let win_cards: Vec<Vec<usize>> = (0..num_win.len())
        .map(|x| if num_win[x] > 0 {(x+1..x+1+num_win[x]).collect()} else {Vec::new()})
        .collect();
    let mut card_totals: Vec<usize> = vec![1; num_win.len()];
    (0..num_win.len()).for_each(|x| for card in &win_cards[x] {card_totals[*card]+=1*card_totals[x]});
    let part_two_total: usize = card_totals.iter().sum();
    println!("Part 2 sum: {}", part_two_total);
}

fn load_input(file_name: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let contents: String = fs::read_to_string(file_name).unwrap();
    let newline: &str = "\r\n";
    let lines_raw: Vec<String> = contents.strip_suffix(newline).unwrap()
        .split(newline)
        .map(|x| x.to_string())
        .collect();
    // strip out 'Card N: '
    let lines_no_card: Vec<String> = lines_raw.iter()
        .map(|x| x[x.find(':').unwrap()+1..x.len()].to_owned()).collect();
    // get winning array
    let return_one: Vec<Vec<usize>> = lines_no_card.iter()
        .map(|x| x[0..x.find('|').unwrap()].trim().split(' ')
            .filter(|w| !w.is_empty())
            .map(|y| y.parse::<usize>().unwrap()).collect()).collect();
    // get number array
    let return_two: Vec<Vec<usize>> = lines_no_card.iter()
        .map(|x| x[x.find('|').unwrap()+1..x.len()].trim().split(' ')
            .filter(|w| !w.is_empty())
            .map(|y| y.parse::<usize>().unwrap()).collect()).collect();
    (return_one, return_two)
}