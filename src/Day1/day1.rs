
pub mod day1 {
    use std::fs;
    use std::str::Split;
    pub fn solve(){
        part1();
        part2();
    }
     fn part2() {
        let mut input = fs::read_to_string("src/day1/input.txt")
            .expect("Should have been able to read the file");
        //sanitize the data
        input = input.replace("nine", "n9e");
        input = input.replace("eight", "e8t");
        input = input.replace("seven", "s7n");
        input = input.replace("six", "s6x");
        input = input.replace("five", "f5e");
        input = input.replace("four", "f4r");
        input = input.replace("three", "t3e");
        input = input.replace("two", "t2o");
        input = input.replace("one", "o1e");
        input = input.replace("zero", "z0o");


        let mut input_line: Split<&str> = input.split("\n");
        let mut sum: i32 = 0;
        let mut output: String = String::new();

        for line in input_line {
            let first = line.chars().find(|&c| c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5'
                || c == '6' || c == '7' || c == '8' || c == '9'
            ).unwrap();

            // first
            output.push(first);
            let last = line.chars().rfind(|&c| c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9').unwrap();
            // println!("Found: {}", last);
            output.push(last);
            sum += output.parse::<i32>().unwrap();
            output.clear()
        }
        println!("Day 1 part 1: {sum}");
        //54706
    }


    fn part1() {
        let input = fs::read_to_string("src/day1/input.txt")
            .expect("Should have been able to read the file");
        let input_line = input.split("\n");
        let mut sum: i32 = 0;
        let mut output = String::new();
        for line in input_line {

            let first = line.chars().find(|&c| c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5'
                || c == '6' || c == '7' || c == '8' || c == '9'
            ).unwrap();

            // first
            output.push(first);
            let last = line.chars().rfind(|&c| c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9').unwrap();
            // println!("Found: {}", last);
            output.push(last);
            sum += output.parse::<i32>().unwrap();
            output.clear()
        }
        println!("Day 1 Part 2: {sum}");
    }
}