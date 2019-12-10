use crate::icc::icc::{new, Computer};
use std::fs;

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    let ans_1 = part_1(&inputs);
    println!("Part 1: {}", ans_1);
    let (noun, verb) = part_2(&inputs);
    println!(
        "Part 2: noun {} and verb {} -> {}",
        noun,
        verb,
        noun * 100 + verb
    );
}

fn load_inputs(file_name: &str) -> Vec<i64> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn part_1(inputs: &Vec<i64>) -> i64 {
    let mut instructions = inputs.clone();
    instructions[1] = 12;
    instructions[2] = 2;
    let mut computer = new(instructions.len(), &instructions);
    assert_eq!(computer.execute(), 0);
    return computer.get_memory_at(0);
}

fn part_2(inputs: &Vec<i64>) -> (i64, i64) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions = inputs.clone();
            instructions[1] = noun;
            instructions[2] = verb;
            let mut computer = new(instructions.len(), &instructions);
            if computer.execute() != 0 {
                continue;
            }
            if computer.get_memory_at(0) == 19690720 {
                return (noun, verb);
            }
        }
    }
    panic!("No answer found for part 2 :(");
}
