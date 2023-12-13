use regex::Regex;
use std::fs;
use num::integer::lcm;

struct Node {
    value: usize,
    left: usize,
    right: usize,
}

struct Instructions {
    template: Vec<bool>, // false='R', true='L'
    pointer: usize,
}

impl Instructions {
    fn get_direction(&mut self) -> bool {
        let out_val: bool = self.template[self.pointer];
        self.pointer += 1;
        if self.pointer == self.template.len() {
            self.pointer = 0;
        };
        out_val
    }
}

fn main() {
    // load input
    let input: (String, Vec<Vec<String>>) = load_input("input.txt");
    // create instructions struct
    let mut inst: Instructions = Instructions {
        template: input
            .0
            .chars()
            .map(|x| if x == 'R' { false } else { true })
            .collect(),
        pointer: 0,
    };
    // create list of nodes (no references yet)
    let mut nodes: Vec<Node> = input.1.iter().map(|x|
        Node {
            value: str_to_num(&x[0]),
            left: 0,
            right: 0,
        }).collect();
    // fill out references
    for i in 0..input.1.len() {
        // find index of left node
        let idx_left: usize = nodes.iter()
            .position(|f| f.value == str_to_num(&input.1[i][1]))
            .unwrap();
        // assign
        nodes[i].left = idx_left;
        // find index of right node
        let idx_right: usize = nodes.iter()
            .position(|f| f.value == str_to_num(&input.1[i][2]))
            .unwrap();
        // assign
        nodes[i].right = idx_right;
    }
    // part 1
    let mut cur_index: usize = nodes.iter()
        .position(|x| x.value == str_to_num("AAA"))
        .unwrap();
    let mut num_its: usize = 0;
    let end_num: usize = str_to_num("ZZZ");
    while nodes[cur_index].value != end_num {
        // get next node
        cur_index = next_node(cur_index, &nodes, &mut inst);
        num_its += 1;
    }
    println!("Part 1: {} jumps", num_its);
    // part 2
    let start_idx: Vec<usize> = (0..nodes.len()).filter(|&x| nodes[x].value % 26 == 0).collect();
    let mut loop_hits: Vec<usize> = vec![];
    for s in start_idx {
        // calc loop
        let loop_result: (Vec<usize>, usize) = find_loop(s, &nodes, &mut inst);
        // find nodes that end in Z
        let loop_len: usize = loop_result.0.len() - loop_result.1;
        let idx_Z: Vec<usize> = (0..loop_result.0.len())
            .filter(|&x| nodes[loop_result.0[x]].value % 26 == 25)
            .collect();
        assert!(idx_Z.len() == 1);     // is this coincidence, or intentional by the puzzle designers?
        assert!(idx_Z[0] == loop_len); // see comment above
        loop_hits.push(idx_Z[0]);
    }
    // get least common multiple of Z locations
    let mut lcm_result: usize = 1;
    for p in loop_hits {
        lcm_result = lcm(lcm_result, p);
    }
    println!("Part 2: {} jumps", lcm_result);
}

fn find_loop(idx_start: usize, nodes: &Vec<Node>, inst: &mut Instructions) -> (Vec<usize>, usize) {
    // set instructions to 0
    inst.pointer = 0;
    // initialize index var
    let mut cur_idx: usize = idx_start.clone();
    // create breadcrumb trail vecs
    let mut history: Vec<(usize, usize)> = vec![];
    // add the starting point
    history.push((cur_idx, inst.pointer));
    while true {
        // get the next node
        cur_idx = next_node(cur_idx, nodes, inst);
        // check if we've already been to this node
        if node_is_in_history((cur_idx, inst.pointer), &history) {
            break;
        } else {
            // add node to history
            history.push((cur_idx, inst.pointer));
        }
    }
    let repeat_pos = node_position_in_history((cur_idx, inst.pointer), &history);
    (history.iter().map(|x| x.0).collect(), repeat_pos)
}

fn node_is_in_history(in_node: (usize, usize), hist: &Vec<(usize, usize)>) -> bool {
    hist.iter().any(|x| x.0 == in_node.0 && x.1 == in_node.1)
}

fn node_position_in_history(in_node: (usize, usize), hist: &Vec<(usize, usize)>) -> usize {
    hist.iter().position(|x| x.0 == in_node.0 && x.1 == in_node.1).unwrap()
}


fn next_node(idx_current: usize, nodes: &Vec<Node>, inst: &mut Instructions) -> usize {
    // get current node
    let cur_node: &Node = &nodes[idx_current];
    // get next instruction
    let move_left: bool = inst.get_direction();
    // get index
    if move_left {
        cur_node.left
    } else {
        cur_node.right
    }
}

fn next_nodes(idx_current: Vec<usize>, nodes: &Vec<Node>, inst: &mut Instructions) -> Vec<usize> {
    let mut output_nodes = idx_current.clone();
    let move_left: bool = inst.get_direction();
    // get each index
    for i in 0..output_nodes.len() {
        if move_left {
            output_nodes[i] = nodes[output_nodes[i]].left;
        } else {
            output_nodes[i] = nodes[output_nodes[i]].right;
        }
    }
    output_nodes
}

fn str_to_num(in_str: &str) -> usize {
    let mut out_num: usize = 0;
    let base: usize = 26;
    for (a, b) in in_str.to_string().chars().enumerate() {
        out_num += ((b as usize) - ('A' as usize)) * (base.pow((2 - a).try_into().unwrap()));
    }
    out_num
}

fn load_input(file_name: &str) -> (String, Vec<Vec<String>>) {
    let contents: String = fs::read_to_string(file_name).unwrap();
    let instructions: String = contents[0..contents.find("\r\n").unwrap()].to_string();
    let letters_re: Regex = Regex::new("[A-Z]{3}").unwrap();
    let nodes: Vec<Vec<String>> = contents[contents.find("\r\n").unwrap()..]
        .split("\r\n")
        .filter(|f| !f.is_empty())
        .map(|x| {
            letters_re
                .find_iter(x)
                .map(|y| y.as_str().to_string())
                .collect()
        })
        .collect();
    (instructions, nodes)
}
