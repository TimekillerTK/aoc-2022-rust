use std::{error::{self, Error}, fmt, str::FromStr};

#[derive(Debug)]
struct InvalidHandError {
    source: char,
}

impl error::Error for InvalidHandError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self)
    }
}

impl fmt::Display for InvalidHandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid hand specified: {:?}", self.source)
    }
}

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
    type Error = InvalidHandError;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(InvalidHandError { source: x })
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    player_one: Hand,
    player_two: Hand,
}


impl Round {
    fn outcome(self) -> RoundResult {
        self.player_one.result(self.player_two)
    }

    fn player_one_score(self) -> u32 {
        self.player_one.choice_score() + self.outcome().score()
    }
}

impl FromStr for Round {
    type Err = InvalidHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut chars = s.chars();
        let (Some(player_one), 
             Some(' '),
             Some(player_two), 
             None) = (chars.next(), 
                                    chars.next(), 
                                    chars.next(), 
                                    chars.next()) else {
            return Err(InvalidHandError { source: 's'}); // fix this later
        };
        
        Ok(Self {
            player_one: player_one.try_into()?,
            player_two: player_two.try_into()?
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

pub fn day02() {
    
    println!("------------------------DAY02------------------------");
    // let rounds = include_str!("day02-data.txt")
    //     .lines()
    //     .map(|x| x.parse::<Round>());
    // for round in rounds {
    //     let round = round.unwrap();
    //     println!("{:?}: Result: {:?}, Player One Score: {}", round, round.outcome(), round.player_one_score())
    // }
    
    let total_score = include_str!("day02-data.txt")
        .lines()
        .map(|x| x.parse::<Round>())
        .map(|y| y.unwrap().player_one_score())
        .sum::<u32>();

    println!("Total Score would be: {}", total_score);
    // println!("Top 3 elves are carrying: {:?} and the sum is: {}", top3, sum_of_top3);
    println!("-----------------------------------------------------\n");

} 