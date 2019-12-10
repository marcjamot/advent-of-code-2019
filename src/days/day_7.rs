use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    part_1(&inputs);
}

fn load_inputs(file_name: &str) -> Vec<i32> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn part_1(memory: &Vec<i32>) {
    let permutations = get_permutations();

    for inputs in permutations {
        // println!("Inputs: {:?}", inputs);

        let (tx_0, rx_0) = channel();
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();
        let (tx_3, rx_3) = channel();
        let (tx_4, rx_4) = channel();

        tx_0.send(inputs[0]).expect("tx_0");
        tx_1.send(inputs[1]).expect("tx_1");
        tx_2.send(inputs[2]).expect("tx_2");
        tx_3.send(inputs[3]).expect("tx_3");
        tx_4.send(inputs[4]).expect("tx_4");

        tx_0.send(0).expect("tx_0 init");

        let m_0 = memory.clone();
        let t_0 = thread::Builder::new().name("0".to_string()).spawn(move || {
            execute("0", m_0, rx_0, tx_1, false);
        });
        let m_1 = memory.clone();
        let t_1 = thread::Builder::new().name("1".to_string()).spawn(move || {
            execute("1", m_1, rx_1, tx_2, false);
        });
        let m_2 = memory.clone();
        let t_2 = thread::Builder::new().name("2".to_string()).spawn(move || {
            execute("2", m_2, rx_2, tx_3, false);
        });
        let m_3 = memory.clone();
        let t_3 = thread::Builder::new().name("3".to_string()).spawn(move || {
            execute("3", m_3, rx_3, tx_4, false);
        });
        let m_4 = memory.clone();
        let t_4 = thread::Builder::new().name("4".to_string()).spawn(move || {
            execute("4", m_4, rx_4, tx_0, true);
        });

        t_0.unwrap().join().expect("t_0");
        t_1.unwrap().join().expect("t_1");
        t_2.unwrap().join().expect("t_2");
        t_3.unwrap().join().expect("t_3");
        t_4.unwrap().join().expect("t_4");
    }
}

fn get_permutations() -> Vec<Vec<i32>> {
    let mut permutations = Vec::new();
    for i0 in 5..10 {
        for i1 in 5..10 {
            if i0 == i1 {
                continue;
            }
            for i2 in 5..10 {
                if i0 == i2 || i1 == i2 {
                    continue;
                }
                for i3 in 5..10 {
                    if i0 == i3 || i1 == i3 || i2 == i3 {
                        continue;
                    }
                    for i4 in 5..10 {
                        if i0 == i4 || i1 == i4 || i2 == i4 || i3 == i4 {
                            continue;
                        }
                        permutations.push(vec![i0, i1, i2, i3, i4]);
                    }
                }
            }
        }
    }
    return permutations;
}

fn execute(
    name: &str,
    mut memory: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
    print_last: bool,
) -> i8 {
    let mut last: i32 = 0;
    let mut p = 0;

    loop {
        let (opcode, am, bm, _) = parse_instruction(memory[p]);
        // println!(
        //     "{} - {} -> op[{}] a[{}] b[{}]",
        //     name, memory[p], opcode, am, bm
        // );
        match opcode {
            // Addition
            1 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = a + b;
                p += 4;
            }
            // Multiplication
            2 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = a * b;
                p += 4;
            }
            // Get input
            3 => {
                let r = memory[p + 1] as usize;
                let v = input.recv().unwrap();
                memory[r] = v;
                // println!("{} - INPUT: {} <- {}", name, r, v);
                p += 2;
            }
            // Print
            4 => {
                let a = get_value(&memory, am, memory[p + 1]);
                last = a;
                output.send(a);
                // println!("{} - LOG: {}", name, a);
                p += 2;
            }
            // Jump if true
            5 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]) as usize;
                if a != 0 {
                    p = b;
                } else {
                    p += 3;
                }
            }
            // Jump if false
            6 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]) as usize;
                if a == 0 {
                    p = b;
                } else {
                    p += 3;
                }
            }
            // Less than
            7 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = if a < b { 1 } else { 0 };
                p += 4;
            }
            // Equals
            8 => {
                let a = get_value(&memory, am, memory[p + 1]);
                let b = get_value(&memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = if a == b { 1 } else { 0 };
                p += 4;
            }
            99 => {
                if print_last {
                    println!("{} - Last: {}", name, last);
                }
                // println!("{} - 99", name);
                return 0;
            }
            _ => {
                println!("{} - -1", name);
                return -1;
            }
        }
    }
}

fn parse_instruction(mut instruction: i32) -> (i32, i32, i32, i32) {
    let opcode = instruction % 100;
    instruction /= 100;
    let am = instruction % 10;
    instruction /= 10;
    let bm = instruction % 10;
    instruction /= 10;
    let rm = instruction % 10;
    return (opcode, am, bm, rm);
}

fn get_value(memory: &Vec<i32>, mode: i32, parameter: i32) -> i32 {
    let v: i32;
    if mode == 1 {
        v = parameter;
    } else {
        v = memory[parameter as usize];
    }
    // println!("  get_value(memory, {}, {}) -> {}", mode, parameter, v);
    return v;
}
