pub mod day6{
    use std::fs;

    pub fn solve() {
        part1();
        part2();
    }
    fn part1() {
        let mut input=fs::read_to_string("src/Day6/input.txt").unwrap();
        input=input.split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let lines =input.split("\n").collect::<Vec<&str>>();
        let mut times_distances:Vec<Vec<&str>>=Vec::new();
        times_distances.push(lines[0].split(" ").collect::<Vec<&str>>());
        times_distances.push(lines[1].split(" ").collect::<Vec<&str>>());
        let mut solutions:Vec<i32>=Vec::new();
        //Evaluate every possible state
        for i in 1..times_distances[0].len() {
            let mut beats=0;

            let currtime:i32=times_distances[0][i].trim().parse().unwrap();
            for time in 1..currtime {
                // println!("dist: {}",time* (currtime-time));
                if time* (currtime-time) >times_distances[1][i].trim().parse::<i32>().unwrap(){
                    beats+=1;
                }
            }
            solutions.push(beats);
        }
        //Calculate solution
        let mut final_solution=1;
        for solution in solutions {
            final_solution*=solution;
        }
        println!("Day 6 Part 1: {}", final_solution);
    }
    fn part2() {
        let mut input=fs::read_to_string("src/Day6/input.txt").unwrap().replace(" ","");
        let lines =input.split("\n").collect::<Vec<&str>>();
        let mut times_distances:Vec<&str>=Vec::new();
        times_distances.push(lines[0].split(":").collect::<Vec<&str>>().last().unwrap());
        times_distances.push(lines[1].split(":").collect::<Vec<&str>>().last().unwrap());
        //Evaluate every possible state
        let mut beats=0;
            let currtime:i64=times_distances[0].trim().parse().unwrap();
            for time in 1..currtime {
                if time* (currtime-time) >times_distances[1].trim().parse::<i64>().unwrap(){
                    beats+=1;
            }
        }
        println!("Day 6 Part 2: {}", beats);
    }
}

