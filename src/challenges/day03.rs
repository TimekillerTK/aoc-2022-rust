use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

const DATA: &str = include_str!("day03-data.txt");

pub fn day03() -> anyhow::Result<()> {

    let mut p1_output = 0;
    let lines = DATA.lines();

    for line in lines {

        // build vector of items
        let line_vec = line
            .bytes()
            .map(Item::try_from)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        // split vector into half
        let (first_half, 
            second_half) = line_vec.split_at(line.len() / 2);

        // find common item in both halves
        let common_item = first_half
            .iter()
            .filter(|&x| 
                second_half.iter()
                .any(|y| x == y))
            .next()
            .map(|x| *x)
            .unwrap();

        // get the common item's priority & sum
        p1_output = p1_output + &common_item.priority;

    };


    let lines = DATA.lines().collect::<Vec<_>>();

    // build vector of vectors
    let vector_of_vectors = lines
        .iter()
        .map(|x| 
            x.bytes()
            .map(Item::try_from)
            .map(|x| x.unwrap())
            .collect::<Vec<Item>>())
        .collect::<Vec<Vec<Item>>>();

    let p2_output: u32 = vector_of_vectors
        .chunks(3)
        .map(|x| {
            let (x1, x2, x3) = (&x[0], &x[1], &x[2]);
            x1
                .iter()
                .find(|a| x2.contains(*a) && x3.contains(*a))
                .unwrap()
                .priority

        })
        .sum();
        
    println!("------------------------DAY03------------------------");
    println!("Part One: Priority sum would be: {p1_output}");
    println!("Part Two: Sum of priorities is: {p2_output}");
    println!("-----------------------------------------------------\n");

    Ok(())
}