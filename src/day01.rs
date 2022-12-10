use itertools::Itertools;

pub fn day01() {

    // Save file on the binary, because why not
    let calories = include_str!("calories.txt")
        .lines()
        .collect::<Vec<_>>();

    let groups = calories
        .split(|x| x.is_empty())
        .collect::<Vec<_>>();

    let calorie_groups = groups
        .iter()
        .map(|x| {
            x.iter()
            .map(|y| y.parse::<u16>().ok().unwrap())
            .sum::<u16>()
        })
        .collect::<Vec<_>>();
    
    let sth = calorie_groups
        .iter()
        .sorted_by_key(|&x| u16::MAX - x)
        .take(3)
        .collect::<Vec<_>>();

    println!("Elf carrying the most calories is carrying: {}", sth[0]);
    println!("Top 3 elves are carrying: {:?}", sth);
    

}