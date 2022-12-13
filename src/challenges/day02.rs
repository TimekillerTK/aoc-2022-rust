use std::{error, fmt, str::FromStr};
use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn choice_score(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn result(self, player_two: Hand) -> RoundResult {
        match (self, player_two) {
            (Hand::Rock, Hand::Rock) => RoundResult::Draw,
            (Hand::Rock, Hand::Paper) => RoundResult::Lose,
            (Hand::Rock, Hand::Scissors) => RoundResult::Win,
            (Hand::Paper, Hand::Rock) => RoundResult::Win,
            (Hand::Paper, Hand::Paper) => RoundResult::Draw,
            (Hand::Paper, Hand::Scissors) => RoundResult::Lose,
            (Hand::Scissors, Hand::Rock) => RoundResult::Lose,
            (Hand::Scissors, Hand::Paper) => RoundResult::Win,
            (Hand::Scissors, Hand::Scissors) => RoundResult::Draw,
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = anyhow::Error;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(anyhow!("Invalid character: {x}"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    player: Hand,
    opponent: Hand,
}


impl Round {
    fn player_one_outcome(self) -> RoundResult {
        self.player.result(self.opponent)
    }

    fn player_one_score(self) -> u32 {
        self.player.choice_score() + self.player_one_outcome().score()
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut chars = s.chars();
        let (Some(player_two), 
             Some(' '),
             Some(player_one), 
             None) = (chars.next(), 
                                    chars.next(), 
                                    chars.next(), 
                                    chars.next()) else {
            return Err(anyhow!("Invalid Input: {s}"));
        };
        
        Ok(Self {
            player: player_one.try_into()?,
            opponent: player_two.try_into()?
        })

    }
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Lose,
    Draw,
} 

impl RoundResult {
    fn score(self) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0
        }
    }
}

pub fn day02() -> anyhow::Result<()>{
    
    let total_score = include_str!("day02-data.txt")
        .lines()
        .map(|x| x.parse::<Round>())
        .map(|y| y.unwrap().player_one_score())
        .sum::<u32>();

    println!("------------------------DAY02------------------------");
    println!("Total Score would be: {}", total_score);
    // println!("Top 3 elves are carrying: {:?} and the sum is: {}", top3, sum_of_top3);
    println!("-----------------------------------------------------\n");

    Ok(())

} 