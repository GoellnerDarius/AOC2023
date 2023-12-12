pub mod day10 {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fs;
    use petgraph::algo::{connected_components, dijkstra};
    use petgraph::algo::kosaraju_scc;
    use petgraph::Graph;
    use petgraph::graph::{DiGraph, node_index, NodeIndex};
    use petgraph::visit::{Dfs, Walker};


    pub fn solve() {
        part1();
        part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day10/input.txt").unwrap();
        let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect::<Vec<&str>>();
        let mut coords_entry: HashMap<(i32, i32), NodeIndex> = HashMap::new();
        let mut graph: Graph<char, i32> = Graph::new();
        let mut startpoint: (i32, i32) = (-1, -1);
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if (lines[i].chars().nth(j).unwrap()) == 'S' {
                    startpoint = (i as i32, j as i32);
                }
                coords_entry.insert((i as i32, j as i32), graph.add_node((lines[i].chars().nth(j).unwrap())));
            }
        }

        //Connect nodes | =Up to Down, - = Left to Right, L = Up to right, J = Up to left, 7 = Down to left, F = Down to right, . =invalid
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let node1 = lines[i].chars().nth(j).unwrap();
                if node1 != '.' {

                    //Connect up
                    if (i > 0) {
                        let node2 = lines[i - 1].chars().nth(j).unwrap();
                        if is_connected(node1, node2, Direction::Up) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 - 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Down
                    if (i < lines.len() - 1) {
                        let node2 = lines[i + 1].chars().nth(j).unwrap();
                        if is_connected(node1, node2, Direction::Down) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 + 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Left
                    if (j > 0) {
                        let node2 = lines[i].chars().nth(j - 1).unwrap();
                        if is_connected(node1, node2, Direction::Left) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 - 1)).unwrap(), 1);
                        }
                    }
                    if (j < lines[i].len() - 1) {
                        let node2 = lines[i].chars().nth(j + 1).unwrap();
                        if is_connected(node1, node2, Direction::Right) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 + 1)).unwrap(), 1);
                        }
                    }
                }
            }
        }
        //Generated Graph
        println!("{},{}", startpoint.0, startpoint.1);
        let distances = dijkstra(&graph, *coords_entry.get(&startpoint).unwrap(), None, |_| 1);

        let mut max = 0;
        for distance in distances {
            if distance.1 > max {
                max = distance.1;
            }
        }

        println!("Day 10 Part 1: {max}")
        //6866 Low
        //6870
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn is_connected(mut c1: char, mut c2: char, direction: Direction) -> bool {
        if c1 == 'S' {
            c1 = '|';
        }
        if c2 == 'S' {
            c2 == '|';
        }
        // println!("{c1}, {c2}",);
        if (c1 == 'S' && c2 != '.') {
            return true;
        }
        match direction {
            Direction::Up => match c1 {
                '|' => c2 == '|' || c2 == 'F' || c2 == '7',
                'L' => c2 == '|' || c2 == 'F' || c2 == '7',
                'J' => c2 == '|' || c2 == 'F' || c2 == '7',
                '-' => false,
                _ => false,
            },
            Direction::Down => match c1 {
                '|' => c2 == '|' || c2 == 'L' || c2 == 'J',
                '7' => c2 == '|' || c2 == 'L' || c2 == 'J',
                'F' => c2 == '|' || c2 == 'L' || c2 == 'J',
                _ => false,
            },
            Direction::Left => match c1 {
                'J' => c2 == '-' || c2 == 'F' || c2 == 'L',
                '7' => c2 == '-' || c2 == 'F' || c2 == 'L',
                '-' => c2 == '-' || c2 == 'F' || c2 == 'L',
                _ => false,
            },
            Direction::Right => match c1 {
                'L' => c2 == '-' || c2 == 'J' || c2 == '7',
                'F' => c2 == '-' || c2 == 'J' || c2 == '7',
                '-' => c2 == '-' || c2 == 'J' || c2 == '7',
                _ => false,
            },
        }
    }

    fn part2() {
        let input = fs::read_to_string("src/Day10/input.txt").unwrap();
        let mut lines: Vec<&str> = input.lines().map(|line| line.trim()).collect::<Vec<&str>>();
        let mut coords_entry: HashMap<(i32, i32), NodeIndex> = HashMap::new();
        let mut startpoint: (i32, i32) = (-1, -1);
        let mut dot_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut graph: Graph<String, i32> = Graph::new();
        let mut new_lines:Vec<String>=Vec::new();
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if (lines[i].chars().nth(j).unwrap()) == 'S' {
                    startpoint = (i as i32, j as i32);
                } else if (lines[i].chars().nth(j).unwrap()) == '.' {
                    dot_coords.insert((i, j));
                }
                let result_string = format!("{}{}{}", lines[i].chars().nth(j).unwrap(), i, j);

                coords_entry.insert((i as i32, j as i32), graph.add_node(
                    // (lines[i].chars().nth(j).unwrap())
                    format!("{},{},{}", lines[i].chars().nth(j).unwrap(), i, j)
                ));
            }
        }

        //Connect nodes | =Up to Down, - = Left to Right, L = Up to right, J = Up to left, 7 = Down to left, F = Down to right, . =invalid
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let node1 = lines[i].chars().nth(j).unwrap();
                if node1 != '.' {

                    //Connect up
                    if (i > 0) {
                        let node2 = lines[i - 1].chars().nth(j).unwrap();
                        if is_connected(node1, node2, Direction::Up) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 - 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Down
                    if (i < lines.len() - 1) {
                        let node2 = lines[i + 1].chars().nth(j).unwrap();
                        if is_connected(node1, node2, Direction::Down) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 + 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Left
                    if (j > 0) {
                        let node2 = lines[i].chars().nth(j - 1).unwrap();
                        if is_connected(node1, node2, Direction::Left) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 - 1)).unwrap(), 1);
                        }
                    }
                    if (j < lines[i].len() - 1) {
                        let node2 = lines[i].chars().nth(j + 1).unwrap();
                        if is_connected(node1, node2, Direction::Right) {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 + 1)).unwrap(), 1);
                        }
                    }
                } else {
                    if (i > 0) {
                        let node2 = lines[i - 1].chars().nth(j).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 - 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Down
                    if (i < lines.len() - 1) {
                        let node2 = lines[i + 1].chars().nth(j).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 + 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Left
                    if (j > 0) {
                        let node2 = lines[i].chars().nth(j - 1).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 - 1)).unwrap(), 1);
                        }
                    }
                    if (j < lines[i].len() - 1) {
                        let node2 = lines[i].chars().nth(j + 1).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 + 1)).unwrap(), 1);
                        }
                    }
                }
            }
        }
        //Generated Graph
        println!("{},{}", startpoint.0, startpoint.1);
        let distances = dijkstra(&graph, *coords_entry.get(&startpoint).unwrap(), None, |_| 1);

        let mut max = 0;
        for distance in distances {
            if distance.1 > max {
                max = distance.1;
            }
        }
        println!("{}", graph[coords_entry[&(0, 0)]]);
        let x = graph[coords_entry[&(0, 0)]].clone();
        println!("Day 10 Part 1: {max}");
        //6870

        let mut m_loop: HashSet<(usize, usize)> = HashSet::new();
        let mut dfs = Dfs::new(&graph, coords_entry[&startpoint]);

        //Get the coordinates of nodes inside the main loop
        while let Some(nx) = dfs.next(&graph) {
            let cloned_value = (graph.clone())[nx].clone();
            let values: Vec<&str> = cloned_value.split(",").collect();
            m_loop.insert((values[1].parse::<usize>().unwrap(), values[2].parse::<usize>().unwrap()));
        }
        // let test= m_loop.union(&dot_coords.clone()).collect::<HashSet<(usize,usize)>>();
        let test = m_loop.union(&dot_coords.clone()).cloned().collect::<HashSet<(usize, usize)>>();

        for i in 0..lines.len() {
            new_lines.push(lines[i].to_string());
        }
        for i in 0..new_lines.len() {
            for j in 0..new_lines[i].len() {
                if !test.contains(&(i,j)){
                    new_lines[i].replace_range(j..j+1,".");
                }
            }
        }
        for new_line in &new_lines {
            println!("{new_line}")
        }
        println!("AAAAAAAAAAAAAAAAAAAAAAAA{}",test.len());



        //Regernerate the graph
        graph.clear();
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if (lines[i].chars().nth(j).unwrap()) == 'S' {
                    startpoint = (i as i32, j as i32);
                } else if (lines[i].chars().nth(j).unwrap()) == '.' {
                    dot_coords.insert((i, j));
                }
                let result_string = format!("{}{}{}", lines[i].chars().nth(j).unwrap(), i, j);

                coords_entry.insert((i as i32, j as i32), graph.add_node(
                    // (lines[i].chars().nth(j).unwrap())
                    format!("{},{},{}", lines[i].chars().nth(j).unwrap(), i, j)
                ));
            }
        }

        //Connect nodes | =Up to Down, - = Left to Right, L = Up to right, J = Up to left, 7 = Down to left, F = Down to right, . =invalid
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let node1 = new_lines[i].clone().chars().nth(j).unwrap();
                if node1 == '.' {
                    if (i > 0) {
                        let node2 = new_lines[i - 1].clone().chars().nth(j).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 - 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Down
                    if (i < lines.len() - 1) {
                        let node2 = new_lines[i + 1].clone().chars().nth(j).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32 + 1, j as i32)).unwrap(), 1);
                        }
                    }
                    //Connect Left
                    if (j > 0) {
                        let node2 = new_lines[i].clone().chars().nth(j - 1).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 - 1)).unwrap(), 1);
                        }
                    }
                    if (j < lines[i].len() - 1) {
                        let node2 = new_lines[i].clone().chars().nth(j + 1).unwrap();
                        if node2 == '.' {
                            graph.add_edge(*coords_entry.get(&(i as i32, j as i32)).unwrap(), *coords_entry.get(&(i as i32, j as i32 + 1)).unwrap(), 1);
                        }
                    }
                }
            }
        }





        let mut stops_up_down: Vec<i32> = Vec::new();
        let mut stops_left_right: Vec<i32> = Vec::new();
        stops_up_down.push(0);
        stops_left_right.push(0);
        stops_up_down.push((lines.len() - 1) as i32);
        stops_left_right.push((lines[0].len() - 1) as i32);
        let mut entries = 0;
        let mut sum = 0;
        let mut c: i32 = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();




        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if dot_coords.contains(&(i, j)) {
                    dot_coords.remove(&(i, j));
                    let mut dfs = Dfs::new(&graph, coords_entry[&(i as i32, j as i32)]);
                    let mut current_sum = 0;


                    // let mut dfs = Dfs::new(&graph, coords_entry[&(i as i32, j as i32)]);
                    'dfs: while let Some(nx) = dfs.next(&graph) {
                        let cloned_value = (graph.clone())[nx].clone();
                        let values: Vec<&str> = cloned_value.split(",").collect();
                        dot_coords.remove(&(values[1].parse::<usize>().unwrap(), values[2].parse::<usize>().unwrap()));
                        // if !visited.contains(&(values[1].to_string(), values[2].to_string())) {
                        //     visited.insert((values[1].to_string(), values[2].to_string()));
                        // }

                        if lines[values[1].parse::<usize>().unwrap()].chars().nth(values[2].parse::<usize>().unwrap()).unwrap()!='.'{
                            current_sum-=1;
                        }
                        // else {
                        //     c-=1;
                        //     // current_sum=-1;
                        // //    break 'dfs
                        //     continue 'dfs;
                        // }
                        c += 1;
                        if stops_up_down.contains(&values[1].parse::<i32>().unwrap()) {
                            current_sum = 0;
                            break 'dfs;
                        } else if stops_left_right.contains(&values[2].parse::<i32>().unwrap()) {
                            current_sum = 0;
                            break 'dfs;
                        }

                        current_sum += 1;
                    }
                    if (current_sum != 0) {
                        entries += 1;
                    }
                    sum += current_sum;
                }
            }
        }


        println!("{}", lines.len());
        println!("{}", lines[0].len());

        println!("Connected components: {}", c);
        println!("Connected components: {}", sum);
        println!("Connected components: {}", entries);
        //545 High
        //506 High

        //287
    }
}


    