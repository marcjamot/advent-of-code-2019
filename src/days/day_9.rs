use crate::computer;
use crate::computer::Computer;
use crate::reader;
use crate::writer;
use std::fs;

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
    let mut c = computer::new(100000, memory);
    c.set_reader(Box::new(reader::once(1)));
    c.register_writer(Box::new(writer::StdOut {}));
    assert_eq!(0, c.execute());
}

fn part_2(memory: &Vec<i64>) {
    let mut c = computer::new(100000, memory);
    c.set_reader(Box::new(reader::once(2)));
    c.register_writer(Box::new(writer::StdOut {}));
    assert_eq!(0, c.execute());
}
