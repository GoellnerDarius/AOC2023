pub mod day2 {
    use std::collections::HashMap;
    use std::fs;
    use std::io::Split;

    pub fn solve() {
        part1();
    }

    fn part1() {
        let input: String = fs::read_to_string("src/day2/input.txt").unwrap();
        let mut limits: HashMap<String, i32> = HashMap::new();
        let mut curr_max: HashMap<String, i32> = HashMap::new();

        let lines: std::str::Split<&str> = input.split("\n");
        let mut sum: i32 = 0;
        let mut powers: i32 = 0;

        for line in lines {
            let mut rounds = line.split(":");
            let game_index = rounds.next().unwrap().split(" ").last().unwrap();
            let mut is_viable: bool = true;
            curr_max.insert("red".to_string(), 0);
            curr_max.insert("green".to_string(), 0);
            curr_max.insert("blue".to_string(), 0);
            limits.insert("red".to_string(), 12);
            limits.insert("green".to_string(), 13);
            limits.insert("blue".to_string(), 14);

            for round in rounds {
                let drawings = round.split(";");

                for drawing in drawings {
                    let mut colors = drawing.split(",");
                    for color in colors {
                        let mut amount_color_pair = color.split(' ');
                        //if amount of cubes is bigger than the limit of this color
                        if *limits.get(amount_color_pair.clone().last().unwrap().trim()).expect("NO COLOR1")
                            < amount_color_pair.clone().skip(1).next().unwrap().parse::<i32>().unwrap()
                        {
                            is_viable = false;
                        }
                        if *curr_max.get(amount_color_pair.clone().last().unwrap().trim()).expect("NO COLOR")
                            < amount_color_pair.clone().skip(1).next().unwrap().parse::<i32>().unwrap()
                        {
                            *curr_max.get_mut(amount_color_pair.clone().last().unwrap().trim()).unwrap() = amount_color_pair.clone().skip(1).next().unwrap().parse::<i32>().unwrap();
                        }
                    }
                }
                if is_viable
                {
                    sum += game_index.parse::<i32>().unwrap();
                }
                let mut tmpsum = 1;
                tmpsum *= curr_max.get("red").unwrap();
                tmpsum *= curr_max.get("green").unwrap();
                tmpsum *= curr_max.get("blue").unwrap();
                powers += tmpsum;
                *curr_max.get_mut("red").unwrap() = 0;
                *curr_max.get_mut("green").unwrap() = 0;
                *curr_max.get_mut("blue").unwrap() = 0;
            }
        }
        println!("Day 2 Part 1: {sum}");
        println!("Day 2 Part 2: {powers}");
    }
}