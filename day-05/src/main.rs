fn parse_line(line: &str) -> Vec<char> {
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
    //let input = include_str!("test_input.txt");
    let input = include_str!("input.txt");
    let mut init_stacking = Vec::new();
    let mut move_instructions = Vec::new();
    enum State {
        Stacking,
        Moves,
    }
    let mut state = State::Stacking;
    for line in input.lines() {
        if line.is_empty() {
            state = State::Moves;
            continue;
        }
        match state {
            State::Stacking => {
                init_stacking.push(line);
            }
            State::Moves => {
                move_instructions.push(line);
            }
        }
    }
    let move_instructions = move_instructions;
    let num_stacks = init_stacking.pop().unwrap().split_whitespace().count();

    init_stacking.reverse();
    let init_stacking = init_stacking;
    let rows = init_stacking
        .iter()
        .map(|line| parse_line(line))
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
    println!("start stacks:");
    print_stacks(&stacks);
    let mut stacks2 = stacks.clone();

    let mut instructions = Vec::new();
    for inst in move_instructions {
        let parts: Vec<_> = inst
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",")
            .split(",")
            .map(|x| x.to_string())
            .collect();
        let mut parts = parts.iter();
        let num = parts.next().unwrap().parse::<usize>().unwrap();
        let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        instructions.push((num, from, to));
    }
    let instructions = instructions;
    for (num, from, to) in instructions.iter() {
        for _ in 0..*num {
            let the_crate = stacks[*from].pop().unwrap();
            stacks[*to].push(the_crate);
        }
    }
    println!();
    println!("part 1:");
    print_stacks(&stacks);
    for stack in stacks {
        match stack.last() {
            Some(top) => print!("{top}"),
            None => print!("-"),
        }
    }
    println!();

    for (num, from, to) in instructions.iter() {
        let height = stacks2[*from].len();
        let cs: Vec<char> = stacks2[*from].drain(height - *num..).collect();
        for c in cs {
            stacks2[*to].push(c);
        }
    }
    println!();
    println!("part2:");
    print_stacks(&stacks2);
    for stack in stacks2 {
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
