use itertools::Itertools;

pub fn day01() {

    // Save file on the binary, because why not
    let calories = include_str!("day01-data.txt")
        .lines()
        .collect::<Vec<_>>();

    let groups = calories
        .split(|x| x.is_empty())
        .collect::<Vec<_>>();

    let calorie_groups = groups
        .iter()
        .map(|x| {
            x.iter()
            .map(|y| y.parse::<u32>().ok().unwrap())
            .sum::<u32>()
        })
        .collect::<Vec<_>>();
    
    let top3 = calorie_groups
        .iter()
        .sorted_by_key(|&x| u32::MAX - x)
        .take(3)
        .collect::<Vec<_>>();

    let sum_of_top3 = top3
        .iter()
        .fold(0, |sum, &x| sum + x);
        

    println!("------------------------DAY01------------------------");
    println!("Elf carrying the most calories is carrying: {}", top3[0]);
    println!("Top 3 elves are carrying: {:?} and the sum is: {}", top3, sum_of_top3);
    println!("-----------------------------------------------------\n");
    

}