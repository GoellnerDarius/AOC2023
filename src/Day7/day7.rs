pub mod day7 {
    use std::collections::{HashMap, HashSet};
    use std::fs;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum HandTypes {
        Five = 6,
        Four = 5,
        Full = 4,
        Three = 3,
        TwoP = 2,
        OneP = 1,
        HighK = 0,
    }

    #[derive(Debug, Clone)]
    struct HandData {
        cards: Vec<u32>,
        card_rank: HandTypes,
        bid: i32,
    }

    impl HandData {
        fn cmp_cards(&self, other: &HandData) -> std::cmp::Ordering {
            // Compare card values
            let card_value_ordering = self.card_rank.cmp(&other.card_rank);

            // If card values are not equal, return the ordering
            if card_value_ordering != std::cmp::Ordering::Equal {
                return card_value_ordering;
            }

            // If card values are equal, compare cards at each index
            for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
                let card_ordering = self_card.cmp(other_card);

                // If cards are not equal, return the ordering
                if card_ordering != std::cmp::Ordering::Equal {
                    return card_ordering;
                }
            }

            // If all cards are equal, return Equal
            std::cmp::Ordering::Equal
        }
    }

    impl Ord for HandData {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.cmp_cards(other)
        }
    }

    impl PartialOrd for HandData {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Option::from(self.cmp_cards(other))
        }
    }


    impl Eq for HandData {}

    impl PartialEq for HandData {
        fn eq(&self, other: &Self) -> bool {
            if (self.card_rank != other.card_rank)
            {
                return false;
            }
            for i in 0..self.cards.len() {
                if (self.cards[i] != other.cards[i]) {
                    return false;
                }
            }
            return true;
        }
    }


    fn get_hand_type(hand: String, has_jokers: bool) -> HandTypes {
        let chars = map_string_to_int_vec(&hand, has_jokers);


        let mut counts: Vec<i32> = Vec::new();
        let mut jokers: i32 = chars.iter().filter(|&n| *n == 0).count() as i32;
        for card in 2..15 {
            counts.push(chars.iter().filter(|&n| *n == card).count() as i32);
        }
        counts.sort();
        if (jokers > 0) {
            while jokers > 0 {
                for i in (0..counts.len()).rev() {
                    if (counts[i] != 5) {
                        counts[i] += 1;
                        jokers -= 1;
                        break;
                    }
                }
            }
        }


        if (*counts.last().unwrap() == 5) {
            return HandTypes::Five;
        }
        if (*counts.last().unwrap() == 4) {
            return HandTypes::Four;
        }
        if *counts.last().unwrap() == 3 && *counts.get(counts.len() - 2).unwrap() == 2 {
            return HandTypes::Full;
        }
        if (*counts.last().unwrap() == 3) {
            return HandTypes::Three;
        }
        if *counts.last().unwrap() == 2 && *counts.get(counts.len() - 2).unwrap() == 2 {
            return HandTypes::TwoP;
        }
        if *counts.last().unwrap() == 2 {
            return HandTypes::OneP;
        }
        return HandTypes::HighK;
    }

    fn map_string_to_int_vec(hand: &str, hasjokers: bool) -> Vec<u32> {
        let mut data: Vec<u32> = Vec::new();
        let chars = hand.chars();
        if (!hasjokers) {
            for char in chars {
                data.push(match char {
                    '2'..='9' => char.to_digit(10).unwrap(),
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Invalid char{}", char),
                });
            }
        } else {
            for char in chars {
                data.push(match char {
                    '2'..='9' => char.to_digit(10).unwrap(),
                    'T' => 10,
                    'J' => 0,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Invalid char{}", char),
                });
            }
        }

        // data.sort();
        // data.reverse();
        return data;
    }

    pub fn solve() {
        part1();
        part2();
    }

    fn part1() {
        let input = fs::read_to_string("src/Day7/input.txt").unwrap();
        let mut lines = input.split("\n");
        let mut hands: Vec<HandData> = Vec::new();
        for line in lines {
            let hand: HandData = HandData {
                cards: map_string_to_int_vec(line.split(" ").next().unwrap(), false),
                card_rank: get_hand_type(line.split(" ").next().unwrap().to_string(), false),
                bid: line.split(" ").last().unwrap().trim().parse::<i32>().expect("couldn't parse bid"),
            };
            hands.push(hand);
        }

        hands.sort();
        let mut sum = 0;
        let mut count = 1;


        for hand in &hands {
            sum += (count) * hand.bid;
            count += 1;
        }
        println!("Day 7 Part 1: {sum}")
        //248453531
    }


    fn part2() {
        let input = fs::read_to_string("src/Day7/input.txt").unwrap();
        let mut lines = input.split("\n");
        let mut hands: Vec<HandData> = Vec::new();
        for line in lines {
            let hand: HandData = HandData {
                cards: map_string_to_int_vec(line.split(" ").next().unwrap(), true),
                card_rank: get_hand_type(line.split(" ").next().unwrap().to_string(), true),
                bid: line.split(" ").last().unwrap().trim().parse::<i32>().expect("couldn't parse bid"),
            };
            hands.push(hand);
        }

        hands.sort();
        let mut sum = 0;
        let mut count = 1;


        for hand in &hands {
            sum += (count) * hand.bid;
            count += 1;
        }
        println!("Day 7 Part 2: {sum}")
        //248453531
    }
}

