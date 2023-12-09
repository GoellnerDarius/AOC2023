pub mod day8 {
    use std::collections::HashMap;
    use std::fs;
    use num_integer::lcm;
    use petgraph::graph::{DiGraph, NodeIndex};
    use petgraph::prelude::EdgeRef;
    use regex::Regex;

    pub fn solve() {
        part1();
        // part2();
        part2_1();
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

    fn part2_1() {
        let input = fs::read_to_string("src/Day8/input.txt").unwrap();
        let lines = input.split("\n").collect::<Vec<&str>>();
        let mut directions = lines[0].chars().collect::<Vec<char>>();
        directions.pop();
        let mut nodes: HashMap<String, String> = HashMap::new();
        let mut starting_nodes: Vec<String> = Vec::new();

        //Genrate map with the nodes
        for i in 2..lines.len() {
            let start = lines[i].split("=").next().unwrap().trim().to_string();
            let l_node = lines[i].split("(").last().unwrap().split(",").next().unwrap().trim().to_string();
            let mut r_node = lines[i].split("(").last().unwrap().split(",").last().unwrap().trim().to_string();
            r_node = (&r_node[0..3]).to_string();

            nodes.insert(start.clone() + "R", r_node);
            nodes.insert(start.clone() + "L", l_node);
            if (start.ends_with("A")) {
                starting_nodes.push(start);
            }
        }

        let mut end_node_count: Vec<u64> = Vec::new();
        for starting_node in starting_nodes {
            let mut count: usize = 0;
            let mut end = starting_node.clone();
            while !end.ends_with("Z") {
                end = nodes.get((end + directions[(count % directions.len())].to_string().as_str()).as_str()).expect("{end}").to_string();
                count += 1;
            }
            end_node_count.push(count as u64);
        }

        println!("Day 8 Part 2: {}",end_node_count.into_iter().fold(1, |acc, x| lcm(acc, x)));
        //21_003_205_388_413

    }

}