use std::{str::FromStr};
use anyhow::{anyhow, Ok};

const DATA: &str = include_str!("day02-data.txt");

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

    fn round_one_result(self, player_two: Hand) -> RoundResult {
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

    fn round_two_hand(self, roundend: RoundResult) -> Hand {
        match (self, roundend) {
            (Hand::Rock, RoundResult::Draw) => Hand::Rock,
            (Hand::Paper, RoundResult::Draw) => Hand::Paper,
            (Hand::Scissors, RoundResult::Draw) => Hand::Scissors,
            (Hand::Rock, RoundResult::Lose) => Hand::Scissors,
            (Hand::Paper, RoundResult::Lose) => Hand::Rock,
            (Hand::Scissors, RoundResult::Lose) => Hand::Paper,
            (Hand::Rock, RoundResult::Win) => Hand::Paper,
            (Hand::Paper, RoundResult::Win) => Hand::Scissors,
            (Hand::Scissors, RoundResult::Win) => Hand::Rock,
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
struct RoundPartOne {
    opponent: Hand,
    player: Hand,
}

impl RoundPartOne {
    fn player_one_outcome(self) -> RoundResult {
        self.player.round_one_result(self.opponent)
    }

    fn player_one_score(self) -> u32 {
        self.player.choice_score() + self.player_one_outcome().score()
    }
}

impl FromStr for RoundPartOne {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut chars = s.chars();
        let (Some(opponent), 
             Some(' '),
             Some(player), 
             None) = (chars.next(), 
                                    chars.next(), 
                                    chars.next(), 
                                    chars.next()) else {
            return Err(anyhow!("Invalid Input: {s}"));
        };
        
        Ok(Self {
            player: player.try_into()?,
            opponent: opponent.try_into()?
        })

    }
}

#[derive(Debug, Clone, Copy)]
struct RoundPartTwo {
    opponent: Hand,
    roundend: RoundResult,
}

impl RoundPartTwo {
    fn player_one_choice(self) -> Hand {
        self.opponent.round_two_hand(self.roundend)
    }

    fn player_one_score(self) -> u32 {
        self.player_one_choice().choice_score() + self.roundend.score()
    }
}

impl FromStr for RoundPartTwo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut chars = s.chars();
        let (Some(opponent),
             Some(' '),
             Some(roundend),
             None) = (chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next()) else {
            return Err(anyhow!("Invalid input: {s}"));
        };

        Ok(Self {
            opponent: opponent.try_into()?,
            roundend: roundend.try_into()?
        })
    }
}

#[derive(Debug, Clone, Copy)]
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

impl TryFrom<char> for RoundResult {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'Y' => Ok(RoundResult::Draw),
            'X' => Ok(RoundResult::Lose),
            'Z' => Ok(RoundResult::Win),
            _ => Err(anyhow!("Invalid character: {value}"))
        }
    }
}

pub fn day02() -> anyhow::Result<()>{
    
    let round_one_score = DATA
        .lines()
        .map(|x| x.parse::<RoundPartOne>())
        .map(|y| y.unwrap().player_one_score())
        .sum::<u32>();

    let round_two_score = DATA
        .lines()
        .map(|x| x.parse::<RoundPartTwo>())
        .map(|y| y.unwrap().player_one_score())
        .sum::<u32>();
    println!("------------------------DAY02------------------------");
    println!("Round One Total Score would be: {}", round_one_score);
    println!("Round Two Total Score would be: {}", round_two_score);
    println!("-----------------------------------------------------\n");

    Ok(())

} 