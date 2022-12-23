use OpCode::*;

#[derive(Debug, Default, PartialEq, Eq)]
enum OpCode {
    #[default]
    Noop,
    Addx,
}

#[derive(Debug, Default)]
struct Instruction {
    opcode: OpCode,
    cycles_remaining: i32,
    value: i32,
}

fn main() {
    // let input = "noop
    // addx 3
    // addx -5
    // ";
    // let input = include_str!("sample_input.txt");
    let input = include_str!("input.txt");

    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
        let mut words = line.split(' ');
        let opcode = words
            .next()
            .expect("expected an opcode mneumonic")
            .to_lowercase();
        let new_inst = if opcode == "addx" {
            let value = words.next().expect("expected value");
            let value: i32 = value.parse().expect("expected integer");
            Instruction {
                opcode: Addx,
                cycles_remaining: 2,
                value,
            }
        } else if opcode == "noop" {
            Instruction {
                opcode: Noop,
                cycles_remaining: 1,
                ..Instruction::default()
            }
        } else {
            unreachable!("only opcodes are addx and noop");
        };
        instructions.push(new_inst);
    }

    let mut cycle = 1;
    let mut x = 1;
    let mut signal_strength_sum = 0;
    let mut instructions = instructions.into_iter();
    let mut cur_inst = instructions.next().unwrap();

    let mut cur_crt_line: Vec<char> = Vec::new();
    loop {
        // println!("@{}, inst={:?}, X={}", cycle, cur_inst, x);
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let signal_strength = cycle * x;
                // println!("@{cycle}, signal strength = {signal_strength}");
                signal_strength_sum += signal_strength;
            }
            _ => {}
        }

        let cur_pixel: i32 = (cycle - 1) % 40;
        if cur_pixel == 0 {
            println!("{}", cur_crt_line.iter().collect::<String>());
            cur_crt_line = Vec::new();
        }
        if cur_pixel.abs_diff(x) <= 1 {
            //print!("cycle {}", cycle);
            cur_crt_line.push('#');
        } else {
            cur_crt_line.push('.');
        }
        // println!(
        //     "@{cycle}/{cur_pixel}: ({x}) {}",
        //     cur_crt_line.iter().collect::<String>()
        // );

        cur_inst.cycles_remaining -= 1;
        if cur_inst.cycles_remaining == 0 {
            if cur_inst.opcode == Addx {
                x += cur_inst.value;
            }
            if let Some(next_inst) = instructions.next() {
                cur_inst = next_inst;
            } else {
                cur_inst = Instruction {
                    opcode: Noop,
                    cycles_remaining: 1,
                    ..Instruction::default()
                };
                if cycle > 260 {
                    break;
                }
                //break;
            }
        }
        cycle += 1;
    }
    println!("signal_strength_sum = {signal_strength_sum}");
}
