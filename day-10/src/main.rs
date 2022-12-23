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
    loop {
        println!("@{}, inst={:?}, X={}", cycle, cur_inst, x);
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let signal_strength = cycle * x;
                println!("@{cycle}, signal strength = {signal_strength}");
                signal_strength_sum += signal_strength;
            }
            _ => {}
        }

        cur_inst.cycles_remaining -= 1;
        if cur_inst.cycles_remaining == 0 {
            if cur_inst.opcode == Addx {
                x += cur_inst.value;
            }
            if let Some(next_inst) = instructions.next() {
                cur_inst = next_inst;
            } else {
                break;
            }
        }
        cycle += 1;
    }
    println!("signal_strength_sum = {signal_strength_sum}");
}
