use num_bigint::BigUint;
use num_bigint::ToBigUint;
use std::{collections::VecDeque, str::FromStr};

type WorryLevel = BigUint;

use Operation::*;
#[derive(Debug, Default)]
enum Operation {
    #[default]
    Unknown,
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Add),
            "*" => Ok(Multiply),
            _ => Err(format!("Unexpected operation {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
enum OperationValue {
    #[default]
    OldValue,
    Number(WorryLevel),
}

impl FromStr for OperationValue {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OperationValue::OldValue),
            _ => {
                let n = s.parse::<WorryLevel>();
                if n.is_ok() {
                    Ok(OperationValue::Number(n.unwrap()))
                } else {
                    Err("expected an i32")
                }
            }
        }
    }
}

enum StressManagement {
    None,
    TakeABreathAndDivideBy3,
}

#[derive(Debug, Default)]
struct Monkey {
    id: usize,
    items: VecDeque<WorryLevel>,
    op: Operation,
    op_value: OperationValue,
    test_divisible_by: WorryLevel,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl Monkey {
    fn has_item(&self) -> bool {
        !self.items.is_empty()
    }

    fn catch_item(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }

    /// # Returns
    /// (monkey item is thrown to, item worry level)
    fn take_turn(&mut self, stress_man: StressManagement) -> (usize, WorryLevel) {
        let item_worry_level = self.items.pop_front().unwrap();
        let op_value = match self.op_value.clone() {
            OperationValue::Number(n) => n.clone(),
            OperationValue::OldValue => item_worry_level.clone(),
        };
        let worry_level_while_examining = match self.op {
            Add => item_worry_level + op_value,
            Multiply => item_worry_level * op_value,
            Unknown => panic!("this monkey doesn't have an operation"),
        };
        let worry_level_once_bored = match stress_man {
            StressManagement::None => worry_level_while_examining,
            StressManagement::TakeABreathAndDivideBy3 => {
                worry_level_while_examining / 3.to_biguint().unwrap()
            }
        };
        let target_monkey = if worry_level_once_bored.clone() % self.test_divisible_by.clone()
            == 0.to_biguint().unwrap()
        {
            self.test_true_monkey
        } else {
            self.test_false_monkey
        };
        (target_monkey, worry_level_once_bored)
    }
}

fn main() {
    // let input = include_str!("sample_input.txt");
    let input = include_str!("input.txt");
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut new_monkey = Monkey::default();
    for (_i, line) in input.lines().enumerate() {
        if line.is_empty() {
            monkeys.push(new_monkey);
            new_monkey = Monkey::default();
        } else if line.starts_with("Monkey") {
            new_monkey = Monkey {
                id: line
                    .split_once(' ')
                    .unwrap()
                    .1
                    .trim_end_matches(':')
                    .parse()
                    .unwrap(),
                ..new_monkey
            };
        } else {
            let (label, value) = line.split_once(": ").unwrap();
            let label = label.trim_start();
            if label == "Starting items" {
                new_monkey = Monkey {
                    items: value.split(", ").map(|x| x.parse().unwrap()).collect(),
                    ..new_monkey
                };
            } else if label == "Operation" {
                let mut op_parts = value.split(' ');
                op_parts.next().expect("new");
                op_parts.next().expect(" = ");
                op_parts.next().expect("old");
                let op: Operation = op_parts.next().expect("operation").parse().unwrap();
                let op_value: OperationValue =
                    op_parts.next().expect("operation value").parse().unwrap();
                new_monkey = Monkey {
                    op,
                    op_value,
                    ..new_monkey
                };
            } else if label == "Test" {
                let test_parts = value.split(' ');
                let n = test_parts
                    .last()
                    .unwrap()
                    .parse()
                    .expect("test should end with i32");
                new_monkey = Monkey {
                    test_divisible_by: n,
                    ..new_monkey
                }
            } else if label == "If true" {
                let true_parts = value.split(' ');
                let n = true_parts.last().unwrap().parse().expect("usize");
                new_monkey = Monkey {
                    test_true_monkey: n,
                    ..new_monkey
                }
            } else if label == "If false" {
                let false_parts = value.split(' ');
                let n = false_parts.last().unwrap().parse().expect("usize");
                new_monkey = Monkey {
                    test_false_monkey: n,
                    ..new_monkey
                }
            } else {
                unreachable!("unexpected label {}", label);
            }
        }
    }
    monkeys.push(new_monkey);
    //println!("monkeys = {:#?}", monkeys);

    let mut monkey_inspect_counts = vec![0u32; monkeys.len()];
    for round in 1..=10000 {
        let monkeys_len = monkeys.len();
        for i in 0..monkeys_len {
            while monkeys[i].has_item() {
                monkey_inspect_counts[i] += 1;
                let (target_monkey_id, item) = monkeys[i].take_turn(StressManagement::None);
                let target_monkey = &mut monkeys[target_monkey_id];
                target_monkey.catch_item(item);
            }
        }

        // println!("After round {}:", round);
        // for m in monkeys.iter() {
        //     println!(
        //         "Monkey {}: {}",
        //         m.id,
        //         m.items
        //             .iter()
        //             .map(|x| format!("{}", x))
        //             .collect::<Vec<_>>()
        //             .join(", ")
        //     )
        // }
        let mut inspect_counts_sorted = monkey_inspect_counts.iter().collect::<Vec<_>>();
        inspect_counts_sorted.sort();
        inspect_counts_sorted.reverse();
        let a = inspect_counts_sorted[0];
        let b = inspect_counts_sorted[1];
        let monkey_business_level = a * b;
        println!("round {round} monkey_business_level = {a} * {b} = {monkey_business_level}");
    }
}
