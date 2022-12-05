fn main() {
    let text = include_str!(r"input.txt");
    let mut calories_by_elf: Vec<u32> = text
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|elf| {
            elf.iter()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    calories_by_elf.sort();
    calories_by_elf.reverse();
    let max = calories_by_elf[0];
    let sum_max3: u32 = calories_by_elf[..3].iter().sum();
    println!("max = {max}");
    println!("max 3 sum = {sum_max3}");
}
