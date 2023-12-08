pub mod day5 {
    use std::collections::HashMap;
    use std::fs;
    use std::time::{Duration, Instant};

    #[repr(usize)]
    enum Indexes {
        DestRangeStart = 0,
        SourceRangeStart = 1,
        Range = 2,
    }

    pub fn solve() {
        part1();
        part2();

    }

    fn part1() {
        // let input = fs::read_to_string("src/Day5/input.txt").unwrap().replace("  ", " ");
        //
        // let mut lines = input.split("\n").collect::<Vec<&str>>();
        //
        // //Clean the data
        // for line in &mut lines {
        //     *line = line.trim();
        // }
        //
        // let seeds = lines.get(0).unwrap().split(":").last().unwrap().trim().split(" ").collect::<Vec<&str>>();
        // let mut translated_seeds: Vec<&str> = Vec::new();
        // //Vec of each mapping (x to y), vec of each distinct line, vec of beginning end, range
        // let mut mappings: Vec<Vec<Vec<i64>>> = Vec::new();
        // let mut mapping_index = 0;
        // //Get the mappings
        // for mut i in 3..lines.len() {
        //     if lines[i - 2].is_empty() {
        //         mappings.push(Vec::new());
        //         while i < lines.len() && !lines[i].is_empty() {
        //             mappings[mapping_index].push(lines[i]
        //                 .trim()
        //                 .split(" ")
        //                 .map(|s| s.parse::<i64>())
        //                 .filter_map(Result::ok)
        //                 .collect());
        //             // mappings.push(lines[i].trim().split(" ").collect::<Vec<&str>>());
        //             // println!("{}", lines[i - 1]);
        //             i += 1;
        //         }
        //         mapping_index += 1;
        //     }
        // }
        // let mut final_seeds: Vec<i64> = Vec::new();
        // let mut seed_id = 0;
        // for seed in seeds {
        //     seed_id+=1;
        //     let mut iseed: i64 = seed.parse().unwrap();
        //     let mut  mappingid=1;
        //     for mapping in &mappings {
        //         //Check if it is mapped
        //         'entry: for entries in mapping {
        //
        //             if iseed >= entries[Indexes::SourceRangeStart as usize] && iseed < (entries[Indexes::SourceRangeStart as usize] + entries[Indexes::Range as usize]) {
        //                 // println!("{seed}, {}", entries[Indexes::DestRangeStart as usize] - entries[Indexes::SourceRangeStart as usize] + iseed);
        //                 //Translate the mapping
        //                 iseed = entries[Indexes::DestRangeStart as usize] - entries[Indexes::SourceRangeStart as usize] + iseed;
        //                 break 'entry;
        //             }
        //
        //             // println!();
        //         }
        //         mappingid+=1;
        //
        //         // println!();
        //     }
        //     final_seeds.push(iseed);
        // }
        //
        //
        // // for seed in final_seeds {
        // //     println!("{seed}");
        // // }
        // final_seeds.sort();
        // println!("Day 5 Part 1: {}",final_seeds.get(0).unwrap());
    }

    fn part2() {
        let input = fs::read_to_string("src/Day5/input.txt").unwrap().replace("  ", " ");
        let mut lines = input.split("\n").collect::<Vec<&str>>();

        //Clean the data
        for line in &mut lines {
            *line = line.trim();
        }

        let seeds = lines.get(0).unwrap().split(":").last().unwrap().trim().split(" ").collect::<Vec<&str>>();
        //Vec of each mapping (x to y), vec of each distinct line, vec of beginning end, range
        let mut mappings: Vec<Vec<Vec<i64>>> = Vec::new();
        let mut mapping_index = 0;
        //Get the mappings
        for mut i in 3..lines.len() {
            if lines[i - 2].is_empty() {
                mappings.push(Vec::new());
                while i < lines.len() && !lines[i].is_empty() {
                    mappings[mapping_index].push(lines[i]
                        .trim()
                        .split(" ")
                        .map(|s| s.parse::<i64>())
                        .filter_map(Result::ok)
                        .collect());
                    // mappings.push(lines[i].trim().split(" ").collect::<Vec<&str>>());
                    // println!("{}", lines[i - 1]);
                    i += 1;
                }
                mapping_index += 1;
            }
        }
        let mut final_seeds: Vec<i64> = Vec::new();
        for i in (0..seeds.len()).step_by(2) {
            println!("Seed: {}", i);
            let s1: i64 = seeds[i].parse().unwrap();
            let s2: i64 = (seeds[i + 1].parse::<i64>().unwrap())+s1;
            for seed in s1..s2 {
                let mut iseed: i64 = seed;
                for mapping in &mappings {
                    //Check if it is mapped
                    'entry: for entries in mapping {
                        if iseed >= entries[Indexes::SourceRangeStart as usize] && iseed < (entries[Indexes::SourceRangeStart as usize] + entries[Indexes::Range as usize]) {
                            // println!("{seed}, {}", entries[Indexes::DestRangeStart as usize] - entries[Indexes::SourceRangeStart as usize] + iseed);
                            //Translate the mapping
                            iseed = entries[Indexes::DestRangeStart as usize] - entries[Indexes::SourceRangeStart as usize] + iseed;
                            break 'entry;
                        }
                    }
                }
                final_seeds.push(iseed);
            }
        }
        let minimum =final_seeds.iter().min().unwrap();
        //6472060
        println!("Day 5 Part 2: {}", minimum);
    }
}

