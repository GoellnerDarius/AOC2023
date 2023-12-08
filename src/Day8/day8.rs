pub mod day8 {
    use std::collections::HashMap;
    use std::fs;

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
        let lines = input.split("\n").collect::<Vec<&str>>();
        let mut directions = lines[0].chars().collect::<Vec<char>>();
        directions.pop();
        let mut nodes: HashMap<String, String> = HashMap::new();
        let mut start_points: Vec<String> = Vec::new();

        //Genrate map with the nodes
        for i in 2..lines.len() {
            let start = lines[i].split("=").next().unwrap().trim().to_string();
            let l_node = lines[i].split("(").last().unwrap().split(",").next().unwrap().trim().to_string();
            let mut r_node = lines[i].split("(").last().unwrap().split(",").last().unwrap().trim().to_string();
            r_node = (&r_node[0..3]).to_string();

            nodes.insert(start.clone() + "R", r_node);
            nodes.insert(start.clone() + "L", l_node);
            if start.chars().last().unwrap() == 'A' {
                start_points.push(start.trim().to_string());
            }
        }


        let mut end = "AAA".to_string();

        // for start_point in &start_points {
        //     println!("{start_point}")
        // }

        let mut count: usize = 0;

        'outer: loop {
            let mut finished:bool=true;

            for start_point in &mut start_points {
                *start_point = nodes.get(((*start_point).clone() + directions[(count % directions.len())].to_string().as_str()).as_str()).expect("{end}").to_string();
               if(start_point.chars().last().unwrap()!='Z'){
                   finished=false;
               }
            }
            count+=1;
            if(count%1_000_000_000)==0{
                println!("{count}");
            }

            if(finished){
                break 'outer;
            }


            // end = nodes.get((end + directions[(count % directions.len())].to_string().as_str()).as_str()).expect("{end}").to_string();
        }
        println!("Day 8 Part 2: {count}");
    }
}