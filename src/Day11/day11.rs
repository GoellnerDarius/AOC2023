pub mod day11 {
    use std::collections::HashMap;
    use std::fs;
    use std::hash::Hash;
    use std::i8::MIN;
    use std::iter::empty;
    use petgraph::algo::dijkstra;
    use petgraph::graph::{DiGraph, NodeIndex};
    use petgraph::visit::DfsEvent::TreeEdge;

    pub fn solve() {
        // part1();
        part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day11/input.txt").unwrap();
        let mut lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect::<Vec<String>>();

        let mut empty_columns = Vec::new();
        for mut i in 0..lines.len() {
            if (!lines[i].contains("#")) {
                empty_columns.push(i);
            }
        }
        while !empty_columns.is_empty() {
            let index = empty_columns.pop().unwrap();
            lines.insert(index, lines[index].clone());
        }

        for i in 0..lines[0].len() {
            let mut is_empty = true;

            'cols: for j in 0..lines.len() {
                if (lines[j].chars().nth(i).unwrap() == '#') {
                    is_empty = false;
                }
            }
            if is_empty {
                empty_columns.push(i);
            }
        }
        for i in 0..empty_columns.len() {
            let insert_index = empty_columns.pop().unwrap();
            for i in 0..lines.len() {
                lines[i].insert(insert_index, '.');
            }
        }

        let mut galaxies: Vec<(i32, i32)> = Vec::new();
        for i in 0..lines.len() {
            if (lines[i].contains("#")) {
                for (j, char) in lines[i].char_indices() {
                    if char == '#' {
                        galaxies.push((i as i32, j as i32));
                    }
                }
            }
        }
        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in 0..galaxies.len() {
                if i == j {
                    continue;
                }
                let difx = (galaxies[i].0 - galaxies[j].0).abs();
                let dify = (galaxies[i].1 - galaxies[j].1).abs();
                sum += difx + dify;
            }
        }
        println!("{}", sum / 2);
    }

        fn part2() {
            let input = fs::read_to_string("src/Day11/input.txt").unwrap();
            let mut lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect::<Vec<String>>();

            let mut empty_columns = Vec::new();
            let mut empty_rows = Vec::new();
            //Get empty rows
            for mut i in 0..lines.len() {
                if (!lines[i].contains("#")) {
                    empty_rows.push(i);
                }
            }
            //get empty columns
            for i in 0..lines[0].len() {
                let mut is_empty = true;

                for j in 0..lines.len() {
                    if (lines[j].chars().nth(i).unwrap() == '#') {
                        is_empty = false;
                    }
                }
                if is_empty {
                    empty_columns.push(i);
                }
            }

            //save the galaxies
            let mut galaxies: Vec<(i32, i32)> = Vec::new();
            for i in 0..lines.len() {
                if (lines[i].contains("#")) {
                    for (j, char) in lines[i].char_indices() {
                        if char == '#' {
                            galaxies.push((i as i32, j as i32));
                        }
                    }
                }
            }
            let mut sum:u64 = 0;
            let expansion=1000000;
            //sum the manhattan distances
            for i in 0..galaxies.len() - 1 {
                for j in (i + 1)..galaxies.len() {
                    if i == j {
                        continue;
                    }
                    let mut expanded_x = 0;
                    let mut expanded_y = 0;

                    for emptyrow in &empty_rows {
                        let min = galaxies[i].0.min(galaxies[j].0);
                        let max = galaxies[i].0.max(galaxies[j].0);
                        if (min..=max).contains(&(*emptyrow as i32)) {
                            expanded_y += expansion-1;
                        }
                    }

                    for emptycolumn in &empty_columns {
                        let min = galaxies[i].1.min(galaxies[j].1);
                        let max = galaxies[i].1.max(galaxies[j].1);
                        if (min..=max).contains(&(*emptycolumn as i32)) {
                            expanded_x += expansion-1;
                        }
                    }

                    let min = galaxies[i].1.min(galaxies[j].1);
                    let max = galaxies[i].1.max(galaxies[j].1);
                    let difx = max - min;

                    let min = galaxies[i].0.min(galaxies[j].0);
                    let max = galaxies[i].0.max(galaxies[j].0);
                    let dify = max - min;

                    // Add the expanded distances only once for each pair
                    sum += (difx + expanded_x + dify + expanded_y) as u64;
                }
            }
            println!("{sum}")
            //1139825014 Low
            //512240933238
        }

    fn part2_1() {
        let input = fs::read_to_string("src/Day11/input.txt").unwrap();
        let mut lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect::<Vec<String>>();
        let growth = 2;
        let mut empty_columns = Vec::new();
        for mut i in 0..lines.len() {
            if (!lines[i].contains("#")) {
                lines[i] = lines[i].replace(".", "1");
            }
        }


        for i in 0..lines[0].len() {
            let mut is_empty = true;

            'cols: for j in 0..lines.len() {
                if (lines[j].chars().nth(i).unwrap() == '#') {
                    is_empty = false;
                }
            }
            if is_empty {
                empty_columns.push(i);
            }
        }
        for i in 0..empty_columns.len() {
            let insert_index = empty_columns.pop().unwrap();
            for i in 0..lines.len() {
                lines[i].insert(insert_index, '1');

                lines[i].remove(insert_index + 1);
            }
        }

        let mut galaxies: Vec<(i32, i32)> = Vec::new();
        for i in 0..lines.len() {
            if (lines[i].contains("#")) {
                for (j, char) in lines[i].char_indices() {
                    if char == '#' {
                        galaxies.push((i as i32, j as i32));
                    }
                }
            }
        }
        let mut sum = 0;

        let mut graph = DiGraph::new();
        let  mut nodes: HashMap<(usize, usize), NodeIndex> = HashMap::new();
        let mut galixies: HashMap<(usize, usize), NodeIndex> = HashMap::new();
        let mut id_weight: HashMap<NodeIndex, char> = HashMap::new();

        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                let node=lines[i].chars().nth(j).unwrap();
                let node_id=graph.add_node(node);

                if(node=='#'){
                    galixies.insert((i,j),node_id);
                }
                nodes.insert((i,j),node_id);
                id_weight.insert(node_id,node);
            }
        }
        for i in 0..lines.len() -1{
            for j in 0..lines[i].len() -1{
                let n1=nodes.get(&(i,j)).unwrap();
                let n2=nodes.get(&(i+1,j)).unwrap();
                let n3=nodes.get(&(i,j+1)).unwrap();
                let mut c1=1;
                let mut c2=1;

                if(lines[i+1].chars().nth(j).unwrap()=='1')
                {
                    c1=growth;
                }
                graph.add_edge(*n1,*n2,c1);
                if(lines[i].chars().nth(j+1).unwrap()=='1')
                {
                    c2=growth;
                }
                graph.add_edge(*n1,*n3,c2);

            }}
        for galixy in &galixies {
            println!("{} {} ",galixy.0.clone().0, galixy.0.clone().1);
            let distances = dijkstra(&graph, *galixy.1, None, |_| 1);
            println!("{}",graph.edge_count());
            for g1 in &galixies {
                if galixy.1==g1.1{
                    continue;
                }
                println!("{}"
               , distances.get(&g1.1).unwrap()

                )

            }


        }


        //
        //     for i in 0..galaxies.len() {
        //         for j in i..galaxies.len() {
        //
        //             let min=galaxies[i].0.min(galaxies[j].0);
        //             let max=galaxies[i].0.max(galaxies[j].0);
        //                 for l in min..max {
        //                     let mul=lines[(galaxies[i].0 as usize)].chars().nth(l as usize).unwrap();
        //                     if(mul=='1'){
        //                         sum+=growth;
        //                     }else { sum +=1; }
        //             }
        //             let min=galaxies[i].1.min(galaxies[j].1);
        //             let max=galaxies[i].1.max(galaxies[j].1);
        //                 for l in min..max {
        //                     let mul=lines[l as usize].chars().nth((galaxies[i].1 as usize)).unwrap();
        //                     if(mul=='1'){
        //                         sum+=growth;
        //                     }else { sum +=1; }
        //             }
        //         }
        //     }
        //
        //     for line in lines {
        //         println!("{line}")
        //     }
        //     println!("{}", sum );
    }
}

