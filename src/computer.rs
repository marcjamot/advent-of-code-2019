use crate::reader::Reader;
use crate::writer::Writer;

const INSTRUCTION_ADD: i64 = 1;
const INSTRUCTION_MUL: i64 = 2;
const INSTRUCTION_READ: i64 = 3;
const INSTRUCTION_WRITE: i64 = 4;
const INSTRUCTION_JUMP_IF_TRUE: i64 = 5;
const INSTRUCTION_JUMP_IF_FALSE: i64 = 6;
const INSTRUCTION_LESS_THAN: i64 = 7;
const INSTRUCTION_EQUALS: i64 = 8;
const INSTRUCTION_EXIT: i64 = 99;

pub trait Computer {
    fn execute(&mut self) -> i8;
    fn get_memory_at(&self, position: usize) -> i64;
    fn register_writer(&mut self, writer: Box<dyn Writer>);
    fn set_reader(&mut self, reader: Box<dyn Reader>);
}

pub struct IntComputer {
    memory: Vec<i64>,
    pointer: usize,
    reader: Option<Box<dyn Reader>>,
    writers: Vec<Box<dyn Writer>>,
}

pub fn new(memory_size: usize, program: &Vec<i64>) -> IntComputer {
    assert!(program.len() <= memory_size);
    let mut memory = vec![0; memory_size];
    for i in 0..program.len() {
        memory[i] = program[i];
    }

    return IntComputer {
        memory: memory,
        pointer: 0,
        reader: Option::None,
        writers: Vec::new(),
    };
}

impl IntComputer {
    fn add(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]);
        let r = self.memory[self.pointer + 3] as usize;
        self.memory[r] = a + b;
        self.pointer += 4;
    }

    fn mul(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]);
        let r = self.memory[self.pointer + 3] as usize;
        self.memory[r] = a * b;
        self.pointer += 4;
    }

    fn read(&mut self) {
        let r = self.memory[self.pointer + 1] as usize;
        let v = if let Some(reader) = &mut self.reader {
            reader.as_mut().read()
        } else {
            panic!("Cannot read, no reader defined")
        };
        self.memory[r] = v;
        self.pointer += 2;
    }

    fn write(&mut self, am: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        for writer in &self.writers {
            writer.as_ref().write(a);
        }
        self.pointer += 2;
    }

    fn jump_if_true(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]) as usize;
        if a != 0 {
            self.pointer = b;
        } else {
            self.pointer += 3;
        }
    }

    fn jump_if_false(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]) as usize;
        if a == 0 {
            self.pointer = b;
        } else {
            self.pointer += 3;
        }
    }

    fn less_than(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]);
        let r = self.memory[self.pointer + 3] as usize;
        self.memory[r] = if a < b { 1 } else { 0 };
        self.pointer += 4;
    }

    fn equals(&mut self, am: i64, bm: i64) {
        let a = get_value(&self.memory, am, self.memory[self.pointer + 1]);
        let b = get_value(&self.memory, bm, self.memory[self.pointer + 2]);
        let r = self.memory[self.pointer + 3] as usize;
        self.memory[r] = if a == b { 1 } else { 0 };
        self.pointer += 4;
    }
}

impl Computer for IntComputer {
    fn execute(&mut self) -> i8 {
        loop {
            let (opcode, am, bm, _) = parse_instruction(self.memory[self.pointer]);
            match opcode {
                INSTRUCTION_ADD => self.add(am, bm),
                INSTRUCTION_MUL => self.mul(am, bm),
                INSTRUCTION_READ => self.read(),
                INSTRUCTION_WRITE => self.write(am),
                INSTRUCTION_JUMP_IF_TRUE => self.jump_if_true(am, bm),
                INSTRUCTION_JUMP_IF_FALSE => self.jump_if_false(am, bm),
                INSTRUCTION_LESS_THAN => self.less_than(am, bm),
                INSTRUCTION_EQUALS => self.equals(am, bm),
                INSTRUCTION_EXIT => return 0,
                _ => return -1,
            }
        }
    }

    fn get_memory_at(&self, position: usize) -> i64 {
        return self.memory[position];
    }

    fn set_reader(&mut self, reader: Box<dyn Reader>) {
        self.reader = Option::from(reader);
    }

    fn register_writer(&mut self, writer: Box<dyn Writer>) {
        self.writers.push(writer);
    }
}

fn parse_instruction(mut instruction: i64) -> (i64, i64, i64, i64) {
    let opcode = instruction % 100;
    instruction /= 100;
    let am = instruction % 10;
    instruction /= 10;
    let bm = instruction % 10;
    instruction /= 10;
    let rm = instruction % 10;
    return (opcode, am, bm, rm);
}

fn get_value(memory: &Vec<i64>, mode: i64, parameter: i64) -> i64 {
    let v: i64;
    if mode == 1 {
        v = parameter;
    } else {
        v = memory[parameter as usize];
    }
    // println!("  get_value(memory, {}, {}) -> {}", mode, parameter, v);
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_add_immediate(a: i64, b: i64) -> bool {
        let mut computer = new(4, &vec![INSTRUCTION_ADD, a, b, 0]);
        assert_eq!(computer.pointer, 0);
        computer.add(1, 1);
        assert_eq!(computer.pointer, 4);
        computer.memory[0] == a + b
    }
    #[test]
    fn test_add_position() {
        let mut computer = new(4, &vec![INSTRUCTION_ADD, 0, 0, 0]);
        assert_eq!(computer.pointer, 0);
        computer.add(0, 0);
        assert_eq!(computer.pointer, 4);
        assert_eq!(computer.memory[0], 2);
    }

    #[quickcheck]
    fn test_mul_immediate(a: i64, b: i64) -> bool {
        let mut computer = new(4, &vec![INSTRUCTION_MUL, a, b, 0]);
        assert_eq!(computer.pointer, 0);
        computer.mul(1, 1);
        assert_eq!(computer.pointer, 4);
        computer.memory[0] == a * b
    }
    #[test]
    fn test_mul_position() {
        let mut computer = new(4, &vec![INSTRUCTION_MUL, 0, 0, 0]);
        assert_eq!(computer.pointer, 0);
        computer.mul(0, 0);
        assert_eq!(computer.pointer, 4);
        assert_eq!(computer.memory[0], 4);
    }

    #[quickcheck]
    fn test_read(v: i64) -> bool {
        struct TestReader {
            v: i64,
        }
        impl Reader for TestReader {
            fn read(&mut self) -> i64 {
                return self.v;
            }
        }
        let mut computer = new(4, &vec![INSTRUCTION_READ, 0]);
        computer.set_reader(Box::from(TestReader { v: v }));
        assert_eq!(computer.pointer, 0);
        computer.read();
        assert_eq!(computer.pointer, 2);
        computer.memory[0] == v
    }

    #[test]
    fn test_write_immediate() {
        struct TestWriter {
            expected: i64,
        }
        impl Writer for TestWriter {
            fn write(&self, value: i64) {
                assert_eq!(self.expected, value);
            }
        }
        let mut computer = new(5, &vec![INSTRUCTION_WRITE, 0]);
        computer.register_writer(Box::from(TestWriter { expected: 0 }));
        computer.register_writer(Box::from(TestWriter { expected: 0 }));
        computer.register_writer(Box::from(TestWriter { expected: 0 }));
        assert_eq!(computer.pointer, 0);
        computer.write(1);
        assert_eq!(computer.pointer, 2);
    }
    #[test]
    fn test_write_position() {
        struct TestWriter {
            expected: i64,
        }
        impl Writer for TestWriter {
            fn write(&self, value: i64) {
                assert_eq!(self.expected, value);
            }
        }
        let mut computer = new(5, &vec![INSTRUCTION_WRITE, 0]);
        computer.register_writer(Box::from(TestWriter { expected: 4 }));
        computer.register_writer(Box::from(TestWriter { expected: 4 }));
        computer.register_writer(Box::from(TestWriter { expected: 4 }));
        assert_eq!(computer.pointer, 0);
        computer.write(0);
        assert_eq!(computer.pointer, 2);
    }

    #[test]
    fn test_jump_if_true() {
        let mut computer = new(4, &vec![INSTRUCTION_JUMP_IF_TRUE, 0, 3, 0]);
        assert_eq!(computer.pointer, 0);
        computer.jump_if_true(0, 0);
        assert_eq!(computer.pointer, 0);
        computer.jump_if_true(1, 1);
        assert_eq!(computer.pointer, 3);
    }

    #[test]
    fn test_jump_if_false() {
        let mut computer = new(4, &vec![INSTRUCTION_JUMP_IF_FALSE, 0, 3, 0]);
        assert_eq!(computer.pointer, 0);
        computer.jump_if_false(1, 0);
        assert_eq!(computer.pointer, 0);
        computer.jump_if_false(0, 1);
        assert_eq!(computer.pointer, 3);
    }

    #[quickcheck]
    fn test_less_than_immediate(a: i64, b: i64) -> bool {
        let mut computer = new(4, &vec![INSTRUCTION_LESS_THAN, a, b, 0]);
        assert_eq!(computer.pointer, 0);
        computer.less_than(1, 1);
        assert_eq!(computer.pointer, 4);
        computer.memory[0] == if a < b { 1 } else { 0 }
    }
    #[test]
    fn test_less_than_position() {
        let mut computer = new(4, &vec![INSTRUCTION_LESS_THAN, 3, 1, 0]);
        assert_eq!(computer.pointer, 0);
        computer.less_than(0, 0);
        assert_eq!(computer.pointer, 4);
        assert_eq!(computer.memory[0], 1);
    }

    #[quickcheck]
    fn test_equals_immediate(a: i64, b: i64) -> bool {
        let mut computer = new(4, &vec![INSTRUCTION_EQUALS, a, b, 0]);
        assert_eq!(computer.pointer, 0);
        computer.equals(1, 1);
        assert_eq!(computer.pointer, 4);
        computer.memory[0] == if a == b { 1 } else { 0 }
    }
    #[test]
    fn test_equals_position() {
        let mut computer = new(5, &vec![INSTRUCTION_EQUALS, 3, 4, 0]);
        assert_eq!(computer.pointer, 0);
        computer.equals(0, 0);
        assert_eq!(computer.pointer, 4);
        assert_eq!(computer.memory[0], 1);
    }
}
