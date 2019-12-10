use std::fs;
use crate::computer;
use crate::computer::{Computer};
use crate::reader::once;
use crate::writer::StdOut;

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    part_1(&inputs);
    part_2(&inputs);
}

fn load_inputs(file_name: &str) -> Vec<i64> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn part_1(memory: &Vec<i64>) {
    let mut computer = computer::new(memory.len(), memory);
    computer.set_reader(Box::new(once(1)));
    computer.register_writer(Box::new(StdOut{}));
    assert_eq!(0, computer.execute());
}

fn part_2(memory: &Vec<i64>) {
    let mut computer = computer::new(memory.len(), memory);
    computer.set_reader(Box::new(once(5)));
    computer.register_writer(Box::new(StdOut{}));
    assert_eq!(0, computer.execute());
}
