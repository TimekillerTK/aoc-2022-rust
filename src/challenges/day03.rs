
use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
struct Item {
    letter: char,
    priority: u8
}

impl TryFrom<u8> for Item {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' => Ok(Item {
                letter: value as char,
                priority: 1 as u8 + (value - b'a')
            }),
            b'A'..=b'Z' => Ok(Item {
                letter: value as char,
                priority: 27 as u8 + (value - b'A')
            }),
            _ => Err(anyhow!("Invalid item: {}", value as char))
        }
    }
}

pub fn day03() -> anyhow::Result<()> {

    let lines = include_str!("day03-data.txt").lines();
    for line in lines {
        // let items = line.bytes().map(Item::try_from).collect::<Result<Vec<_>, _>>()?;
        // let items_to_char = items.iter().map(|x| x.0 as char).collect::<Vec<_>>();

        let (first_half, second_half) = line.split_at(line.len() / 2);
        // println!("{:?}", items_to_char);
        // println!("\nfirsthalf: {:#?}, secondhalf: {:#?}", first_half, second_half);

        let one = first_half
            .bytes()
            .map(Item::try_from)
            .map(|x| x.unwrap().letter)
            .collect::<Vec<_>>();
        // let two = second_half
        //     .bytes()
        //     .map(Item::try_from)
        //     .map(|x| x.unwrap().letter)
        //     .collect::<Vec<_>>();
        let two = second_half
            .bytes()
            .map(Item::try_from)
            .find_map(|x| {
                x.ok().and_then(|y| {
                    one.iter().copied().find(|&z| z == y.letter)
                })
            })
            .expect("????");
        println!("WINNER IS: {}", two)

        // THIS RETURNS A CHAR WHICH IT SHOULDN'T, IT SHOULD RETURN A ITEM
        // ASK ON DISCORD

        // println!(" -> {one:?} /\\/ {two:?}")
    };

    Ok(())
}