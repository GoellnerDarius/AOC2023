pub mod day8 {
    use std::collections::HashMap;
    use std::fs;
    use std::ops::Deref;
    use std::time::Instant;
    use petgraph::Graph;
    use petgraph::graph::{DiGraph, Node, node_index, NodeIndex};
    use petgraph::prelude::EdgeRef;
    use petgraph::visit::NodeRef;
    use regex::Regex;

    pub fn solve() {
        // part1();
        part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day8/input.txt").unwrap();
        let lines = input.split("\n").collect::<Vec<&str>>();
        let mut directions = lines[0].chars().collect::<Vec<char>>();
        directions.pop();
        let mut nodes: HashMap<String, String> = HashMap::new();

        //Genrate map with the nodes
        for i in 2..lines.len() {
            let start = lines[i].split("=").next().unwrap().trim().to_string();
            let l_node = lines[i].split("(").last().unwrap().split(",").next().unwrap().trim().to_string();
            let mut r_node = lines[i].split("(").last().unwrap().split(",").last().unwrap().trim().to_string();
            r_node = (&r_node[0..3]).to_string();

            nodes.insert(start.clone() + "R", r_node);
            nodes.insert(start + "L", l_node);
        }


        let mut end = "AAA".to_string();
        let mut count: usize = 0;
        while end != "ZZZ" {
            end = nodes.get((end + directions[(count % directions.len())].to_string().as_str()).as_str()).expect("{end}").to_string();
            count += 1;
        }
        println!("Day 8 Part 1: {count}");
    }

    fn part2() {
        let input = fs::read_to_string("src/Day8/input.txt").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let directions: Vec<char> = lines[0].chars().collect();

        // let mut nodes: HashMap<String, (&str, &str)> = HashMap::new();
        let mut start_points: Vec<NodeIndex> = Vec::new();
        let mut node_string: HashMap<&str, NodeIndex> = HashMap::new();
        let mut graph = DiGraph::new();
        for i in 2..lines.len() {
            let test = Regex::new(r"[a-zA-Z0-9]{3}").unwrap();
            let nodevec: Vec<&str> = test.captures_iter(lines[i])
                .map(|capture| capture.get(0).unwrap().as_str())
                .take(3) // Take the first 3 matches
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
                start_points.push(*node_string.get(nodevec[0].trim()).unwrap());
            }
        }

        let mut count: usize = 0;

        'outer: loop {
            let mut finished: bool = true;

            for start_point in &mut start_points {
                if (directions[count % directions.len()] == 'R') {
                    *start_point = graph.edges(*start_point).next().unwrap().target();
                } else {
                    *start_point = graph.edges(*start_point).last().unwrap().target();
                }

                if !graph[*start_point].ends_with("Z") {
                    finished = false;
                }
            }
            count += 1;
            if (count % 1_000_000_000) == 0 {
                println!("{count}");
            }

            if (finished) {
                break 'outer;
            }
        }
        println!("Day 8 Part 2: {count}");
        //21_003_205_388_413
    }
}