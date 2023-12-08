pub mod day3 {
    use std::fs;

    pub fn solve() {
        part1();
        part2();
    }

    fn part1() {
        let mut sum = 0;

        let binding = fs::read_to_string("src/Day3/input.txt").unwrap();
        let mut input: Vec<String> = binding.lines().map(String::from).collect();

        let element = ".".to_string().repeat(input[0].len());
        input.insert(0, element.clone());
        input.push(element.clone());
        input = input.iter().map(|line| format!(".{}.", line)).collect();

        for y in 1..input.len() - 1 {
            let prev_line: Vec<char> = input.get(y - 1).expect("Out of bounds Y").chars().collect();
            let current_line: Vec<char> = input.get(y).expect("Out of bounds Y").chars().collect();
            let next_line: Vec<char> = input.get(y + 1).expect("Out of bounds Y").chars().collect();
            let mut startindex: i32 = -1;
            let mut endindex: i32 = -1;
            for x in 1..current_line.len()-1 {


                if current_line.get(x).unwrap().is_digit(10) {

                    if startindex == -1
                    {

                        startindex = x as i32;
                        endindex=x as i32;
                    }else {
                        endindex = x as i32
                    }
                }
                if startindex != -1 {
                    // endindex = x as i32;
                    if(!current_line.get(x+1).unwrap().is_digit(10))
                    {
                        for index in startindex-1..endindex+1 {

                            if (
                                //Above
                                !prev_line.get(index as usize).unwrap().is_digit(10) &&
                                *prev_line.get(index as usize).unwrap() != '.') ||
                                //Below
                                (!next_line.get(index as usize).unwrap().is_digit(10) &&
                                    *next_line.get(index as usize).unwrap() != '.')||
                                    //left
                                (!current_line.get((startindex-1) as usize).unwrap().is_digit(10) &&
                                    *current_line.get((startindex -1)as usize).unwrap() != '.')||
                                //right
                                    (!current_line.get((endindex+1) as usize).unwrap().is_digit(10) &&
                                        *current_line.get((endindex +1)as usize).unwrap() != '.')||
                                //Diagonally top left
                                (!prev_line.get((startindex-1) as usize).unwrap().is_digit(10) &&
                                    *prev_line.get((startindex -1)as usize).unwrap() != '.')||
                                //Diagonally top right
                                (!prev_line.get((endindex+1) as usize).unwrap().is_digit(10) &&
                                    *prev_line.get((endindex +1)as usize).unwrap() != '.')||
                                //Diagonally bottom left
                                (!next_line.get((startindex-1) as usize).unwrap().is_digit(10) &&
                                    *next_line.get((startindex -1)as usize).unwrap() != '.')||
                                //Diagonally bottom right
                                (!next_line.get((endindex+1) as usize).unwrap().is_digit(10) &&
                                    *next_line.get((endindex +1)as usize).unwrap() != '.')
                            {
                                sum += stringtoNumber(&current_line, startindex, endindex);
                                startindex=-1;
                                endindex=-1;
                                break;
                            }

                        }
                        startindex=-1;
                        endindex=-1;
                    }
                }
            }
        }
        println!("Day 3 Part 1: {sum}")
        //544664
    }

    fn stringtoNumber(line: &Vec<char>, start: i32, end: i32) -> i32 {
        if(start==-1||end==-1)
        { return 0; }
        let mut x = String::new();
        for i in start..end+1 {
            let mut tmp = line.get(i as usize).expect("Parse overflow");
            x.push(*tmp);
        }
        return x.parse::<i32>().expect("Couldn't Parse to Number");
    }


    fn part2() {
        let mut sum = 0;

        let binding = fs::read_to_string("src/Day3/input.txt").unwrap();
        let mut input: Vec<String> = binding.lines().map(String::from).collect();
        //Sanitize data
        let element = ".".to_string().repeat(input[0].len());
        input.insert(0, element.clone());
        input.push(element.clone());
        input = input.iter().map(|line| format!(".{}.", line)).collect();

        for y in 1..input.len() - 1 {
            for x in 1..  input.get(y).unwrap().len()-1 {

                if *input.get(y).unwrap().chars().collect::<Vec<char>>().get(x).unwrap()=='*'{
                    sum+= search_number(y,x, &input);
                }
            }
        }
        println!("Day 3 Part 2: {sum}")
    }

    fn search_number(pos_y: usize, pos_x:usize, data: &Vec<String>) -> i32 {
        let mut found:i32=-1;
        let mut numbers:Vec<String> = Vec::new();

        numbers.push(String::new());
        numbers.push(String::new());
         for y in pos_y-1..=(pos_y + 1).min(data.len() - 1) {
             for mut x in (pos_x - 1)..=(pos_x + 1).min(data[y].len() - 1) {
                //found character
                if data.get(y).unwrap().chars().collect::<Vec<char>>().get(x).unwrap().is_digit(10){
                    found+=1;
                    //walk back
                    while data.get(y).unwrap().chars().collect::<Vec<char>>().get(x).unwrap().is_digit(10) {
                        x-=1;
                    }
                    x+=1;
                    while data.get(y).unwrap().chars().collect::<Vec<char>>().get(x).unwrap().is_digit(10) {
                        numbers.get_mut(found as usize).unwrap().push(*data.get(y).unwrap().chars().collect::<Vec<char>>().get(x).unwrap());
                        x+=1;
                    }
                    if(pos_x<x){
                        break;
                    }
                }
            }
        }
        if found==1 {
            return numbers.get(0).unwrap().parse::<i32>().unwrap()*numbers.get(1).unwrap().parse::<i32>().unwrap();

        }
        return 0;
    }
}

