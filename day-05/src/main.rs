fn parse_crate_line(line: &str) -> Vec<char> {
    let mut retval = Vec::new();
    let mut iter = line.chars();

    loop {
        let chars = iter.by_ref().take(3).collect::<Vec<_>>();
        let is_crate = chars[0] == '[';
        let id = if is_crate { chars[1] } else { '-' };
        retval.push(id);
        if iter.next().is_none() {
            break;
        }
    }

    retval
}

fn main() {
    //let input = Ved::from_iter(include_str!("test_input.txt").lines());
    let input = Vec::from_iter(include_str!("input.txt").lines());
    let mut groups = input.split(|line| line.is_empty());
    let mut stacking_input = Vec::from(groups.next().unwrap());
    let instructions_input = Vec::from(
        groups
            .next()
            .expect("expected blank line indicating start of move instructions"),
    );

    //  1   2   3  ... num_stacks
    let num_stacks = stacking_input.pop().unwrap().split_whitespace().count();

    // currently, they're in rows, sky to earth. Reverse so when we put them into stacks,
    // the earth-adjacent ones go in first (the bottom of the stack)
    stacking_input.reverse();
    let stacking_input = stacking_input;
    let rows = stacking_input
        .iter()
        .map(|line| parse_crate_line(line))
        .collect::<Vec<_>>();

    let mut stacks = vec![Vec::new(); num_stacks]; // (0..num_stacks).map(|_| Vec::new()).collect::<Vec<_>>();
    for row in rows.iter() {
        let mut crates = row.iter();
        for i in 0..num_stacks {
            match crates.next() {
                Some(x) => {
                    if *x == '-' {
                        continue;
                    } else {
                        stacks[i].push(*x);
                    }
                }
                None => continue,
            }
        }
    }
    let stacks = stacks;
    println!("start stacks:");
    print_stacks(&stacks);
    let mut stacks_for_part1 = stacks.clone();
    let mut stacks_for_part2 = stacks.clone();
    drop(stacks);

    let mut instructions = Vec::new();
    for inst in instructions_input {
        let parts = inst
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",")
            .split(",")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut parts = parts.iter();
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        instructions.push((num, from, to));
    }
    let instructions = instructions;
    for (num, from, to) in instructions.iter() {
        for _ in 0..*num {
            let the_crate = stacks_for_part1[*from].pop().unwrap();
            stacks_for_part1[*to].push(the_crate);
        }
    }
    println!();
    println!("part 1:");
    print_stacks(&stacks_for_part1);
    for stack in stacks_for_part1 {
        match stack.last() {
            Some(top) => print!("{top}"),
            None => print!("-"),
        }
    }
    println!();

    for (num, from, to) in instructions.iter() {
        let height = stacks_for_part2[*from].len();
        let cs: Vec<char> = stacks_for_part2[*from].drain(height - *num..).collect();
        for c in cs {
            stacks_for_part2[*to].push(c);
        }
    }
    println!();
    println!("part2:");
    print_stacks(&stacks_for_part2);
    for stack in stacks_for_part2 {
        match stack.last() {
            Some(top) => print!("{top}"),
            None => print!("-"),
        }
    }
    println!();

    //println!("move_instructions = {:#?}", move_instructions);
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        println!("{} - {}", i, String::from_iter(stack.iter()));
    }
}
