pub mod day8 {
    use std::collections::HashMap;
    use std::fs;
    use num_integer::lcm;
    use petgraph::graph::{DiGraph, NodeIndex};
    use petgraph::visit::EdgeRef;

    use regex::Regex;

    pub fn solve() {
        part1_2();
    }


    fn part1_2() {
        let input = fs::read_to_string("src/Day8/input.txt").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let directions: Vec<char> = lines[0].chars().collect();
        let mut aaa_index: usize = 0;
        let mut start_points: Vec<NodeIndex> = Vec::new();
        let mut node_string: HashMap<&str, NodeIndex> = HashMap::new();
        let mut graph = DiGraph::new();
        for i in 2..lines.len() {
            let node_regex = Regex::new(r"[a-zA-Z0-9]{3}").unwrap();
            let nodevec: Vec<&str> = node_regex.captures_iter(lines[i])
                .map(|capture| capture.get(0).unwrap().as_str())
                .collect();

            if !node_string.contains_key(nodevec[0].trim()) {
                node_string.insert(nodevec[0].trim(), graph.add_node(nodevec[0].trim()));
            }
            if !node_string.contains_key(nodevec[1].trim()) {
                node_string.insert(nodevec[1].trim(), graph.add_node(nodevec[1].trim()));
            }
            if !node_string.contains_key(nodevec[2].trim()) {
                node_string.insert(nodevec[2].trim(), graph.add_node(nodevec[2].trim()));
            }
            graph.add_edge(*node_string.get(nodevec[0].trim()).unwrap(), *node_string.get(nodevec[1].trim()).unwrap(), ());
            graph.add_edge(*node_string.get(nodevec[0].trim()).unwrap(), *node_string.get(nodevec[2].trim()).unwrap(), ());
            if nodevec[0].ends_with('A') {
                if nodevec[0].eq("AAA") {
                    aaa_index = start_points.len();
                }
                start_points.push(*node_string.get(nodevec[0].trim()).unwrap());
            }
        }

        let mut end_node_count: Vec<u64> = Vec::new();
        for start_point in &mut start_points {
            let mut count: usize = 0;
            let mut end = start_point.clone();
            while !graph[*start_point].ends_with("Z") {
                if (directions[count % directions.len()] == 'R') {
                    *start_point = graph.edges(*start_point).next().unwrap().target();
                } else {
                    *start_point = graph.edges(*start_point).last().unwrap().target();
                }
                count += 1;
            }
            end_node_count.push(count as u64);
        }

        println!("Day 8 Part 1: {}", end_node_count[(aaa_index) as usize]);
        //19_631
        println!("Day 8 Part 2: {}", end_node_count.into_iter().fold(1, |acc, x| lcm(acc, x)));
        //21_003_205_388_413
    }
}