pub mod day4 {
    use std::collections::{HashMap, HashSet};
    use std::fs;

    pub fn solve() {
        // part1();
        part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day4/input.txt").unwrap().replace("  "," " );
        let lines = input.split("\n");
        let mut sum = 0;
        for line in lines {
            let your_numbers = line.split("|").last().unwrap().trim().split(" ").collect::<HashSet<&str>>();
            let winning_numbers = line.split("|").next().unwrap().trim().split(" ").collect::<HashSet<&str>>();
            let mut winning_numbers_count: u32 = winning_numbers.intersection(&your_numbers).cloned().collect::<HashSet<_>>().len() as u32;
            if winning_numbers_count > 0 {
                sum += 2_i32.pow(winning_numbers_count - 1);
            }
        }
        println!("Day 4 Part 1: {sum}")
    }

    fn part2() {
        let mut input = fs::read_to_string("src/Day4/input.txt").unwrap();
        input = input.replace("  ", " ");
        let lines = input.split("\n");
        //Get a table of how many of each crad you have
        let mut card_amounts: HashMap<i32, i32> = HashMap::new();
        for i in 0..lines.clone().collect::<Vec<&str>>().len() {
            card_amounts.insert(i as i32, 1);
        }
        let mut sum = 0;
        let mut index = 0;
        for line in lines
        {
            let your_numbers = line.split("|").last().unwrap().trim().split(" ").collect::<HashSet<&str>>();
            let winning_numbers = line.split("|").next().unwrap().trim().split(" ").collect::<HashSet<&str>>();
            let mut winning_numbers_count: u32 = winning_numbers.intersection(&your_numbers).cloned().collect::<HashSet<_>>().len() as u32;
            if winning_numbers_count > 0 {
                sum += 2_i32.pow(winning_numbers_count - 1);
            }
            let mut temp_index = index + 1;
            //Add the additional Cards
            while winning_numbers_count > 0 {
                if temp_index >= card_amounts.len() {
                    break;
                }
                *card_amounts.get_mut(&(temp_index as i32)).unwrap() += card_amounts.get(&(index as i32)).unwrap().clone();
                temp_index += 1;
                winning_numbers_count -= 1;
            }
            index += 1;

        }
        let mut sum_of_cards = 0;
        //Calculate the number of cards
        for card_amount in &card_amounts {
            sum_of_cards += card_amount.1;
        }
        println!("Day 4 Part 1: {sum}");
        println!("Day 4 Part 2: {sum_of_cards}");
        //8549735
    }
}

