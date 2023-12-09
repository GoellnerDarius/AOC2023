pub mod day9 {
    use std::fs;

    pub fn solve() {
        part1();
        // part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day9/input.txt").unwrap();
        let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
        let mut sum_day1 =0;
        let mut sum_day2 =0;
        for line in lines {
            let mut numbers: Vec<i32> = Vec::new();

            //Parse Numbers
            for number in line.split_whitespace() {
                if let Ok(parsed_num) = number.parse::<i32>() { numbers.push(parsed_num); }
            }

            let mut predicion_lines: Vec<Vec<i32>> = Vec::new();
            predicion_lines.push(numbers);
            'outer: for prediction_index in 0..predicion_lines[0].len() {
                // predicion_lines.push(Vec::new());
                let mut predictions: Vec<i32> = Vec::new();

                for i in 0..predicion_lines[prediction_index].len() - 1 {
                    predictions.push(predicion_lines[prediction_index][i + 1] - predicion_lines[prediction_index][i]);
                }
                predicion_lines.push(predictions);
                if predicion_lines.last().unwrap().iter().all(|&x|x==0) {
                    break 'outer;
                }
            }

            //build the Predictions up
            for i in (0..predicion_lines.len()-1).rev() {
                let forward_predicion = predicion_lines[i].last().unwrap()+predicion_lines[i+1].last().unwrap().clone();
                predicion_lines[i].push(forward_predicion);
                let backward_predicion = predicion_lines[i].first().unwrap() - predicion_lines[i + 1].first().unwrap().clone();
                predicion_lines[i].insert(0, backward_predicion);
            }
            sum_day1 +=predicion_lines[0].last().unwrap();
            sum_day2 +=predicion_lines[0].first().unwrap();
        }
        println!("Day 9 Part 1: {}", sum_day1);
        //1980437560
        println!("Day 9 Part 2: {}", sum_day2);
        //977
    }
}

