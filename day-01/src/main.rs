fn main() {
    let input_file = std::env::args().skip(1).take(1).next().unwrap();
    let text = std::fs::read_to_string(input_file).unwrap();
    let mut elves = Vec::new();
    let mut cur_elf = Box::new(Vec::new());
    for line in text.lines() {
        if line.is_empty() {
            elves.push(cur_elf.to_owned());
            cur_elf = Box::new(Vec::new());
        } else {
            cur_elf.push(line.parse::<u32>().unwrap());
        }
    }
    if !cur_elf.is_empty() {
        // the file probably doesn't end with a blank line. I'm too lazy to look
        elves.push(cur_elf.to_owned());
    }

    let elves = elves;
    let calorie_counts: Vec<_> = elves.iter().map(|e| e.iter().sum::<u32>()).collect();
    let max = calorie_counts.iter().max().unwrap();
    println!("max calorie count is {max}");

    let mut sorted = calorie_counts.clone();
    sorted.sort();
    let sorted = sorted;

    let max3 = &sorted[sorted.len() - 3..];
    let max3_sum: u32 = max3.iter().sum();
    println!("sum of 3 highest calorie carriers is {max3_sum}");
}
