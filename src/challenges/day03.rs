
use anyhow::anyhow;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    letter: char,
    priority: u32
}

impl TryFrom<u8> for Item {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' => Ok(Item {
                letter: value as char,
                priority: (1 as u8 + value - b'a') as u32
            }),
            b'A'..=b'Z' => Ok(Item {
                letter: value as char,
                priority: (27 as u8 + value - b'A') as u32
            }),
            _ => Err(anyhow!("Invalid item: {}", value as char))
        }
    }
}

pub fn day03() -> anyhow::Result<()> {

    let mut priority_sum = 0;
    let lines = include_str!("day03-data.txt").lines();

    for line in lines {

        let (first_half, second_half) = line.split_at(line.len() / 2);

        let one = first_half
            .bytes()
            .map(Item::try_from)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let two = second_half
            .bytes()
            .map(Item::try_from)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let common_item = one
            .iter()
            .filter(|&x| 
                two.iter()
                .any(|y| x == y))
            .next().map(|x| *x)
            .unwrap();

        // println!(" FILTER: {:?}", &common_item.letter);
        priority_sum = priority_sum + &common_item.priority;
    };

    // let lines = include_str!("day03-data.txt")
    //     .lines()
    //     .map(|x| 
    //         x.bytes()
    //         .map(|y| 
    //             Item::try_from(y))
    //         .collect::<Vec<_>>()
    //     )
        
    
    // println!("LINETEST: {lines:#?}");
        
        // .map(|x| {
        //     x.bytes()
        //     .Item::try_from)
        //     .collect::Vec<_>>()
        // })
        // .chunks(3);
    // for line in lines {
    //     println!("Line: {:?}", line)
    //     let test_line = line.split
    // }
    println!("------------------------DAY03------------------------");
    println!("Priority sum would be: {}", priority_sum);
    println!("-----------------------------------------------------\n");

    Ok(())
}