use crate::computer;
use crate::computer::Computer;
use crate::reader;
use crate::writer;
use std::fs;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

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
    let permutations = get_permutations(0);

    let mut output = 0;
    for inputs in permutations {
        let (tx_0, rx_0) = channel();
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();
        let (tx_3, rx_3) = channel();
        let (tx_4, rx_4) = channel();
        let (tx_ans, rx_ans) = channel();

        tx_0.send(inputs[0]).expect("Cannot init tx_0");
        tx_1.send(inputs[1]).expect("Cannot init tx_1");
        tx_2.send(inputs[2]).expect("Cannot init tx_2");
        tx_3.send(inputs[3]).expect("Cannot init tx_3");
        tx_4.send(inputs[4]).expect("Cannot init tx_4");

        tx_0.send(0).expect("Cannot feed first input");

        let mut c_0 = computer::new(memory.len(), memory);
        c_0.set_reader(Box::new(reader::channel(rx_0)));
        c_0.register_writer(Box::new(writer::channel(tx_1)));
        let mut c_1 = computer::new(memory.len(), memory);
        c_1.set_reader(Box::new(reader::channel(rx_1)));
        c_1.register_writer(Box::new(writer::channel(tx_2)));
        let mut c_2 = computer::new(memory.len(), memory);
        c_2.set_reader(Box::new(reader::channel(rx_2)));
        c_2.register_writer(Box::new(writer::channel(tx_3)));
        let mut c_3 = computer::new(memory.len(), memory);
        c_3.set_reader(Box::new(reader::channel(rx_3)));
        c_3.register_writer(Box::new(writer::channel(tx_4)));
        let mut c_4 = computer::new(memory.len(), memory);
        c_4.set_reader(Box::new(reader::channel(rx_4)));
        c_4.register_writer(Box::new(writer::channel(tx_ans)));

        assert_eq!(0, c_0.execute());
        assert_eq!(0, c_1.execute());
        assert_eq!(0, c_2.execute());
        assert_eq!(0, c_3.execute());
        assert_eq!(0, c_4.execute());

        loop {
            let o = rx_ans.recv_timeout(Duration::from_millis(10)).unwrap_or(-1);
            if o == -1 {
                break;
            }
            if output < o {
                output = o;
            }
        }
    }

    println!("Part 1: {}", output);
}

fn part_2(memory: &Vec<i64>) {
    let permutations = get_permutations(5);

    let mut output = 0;
    for inputs in permutations {
        let (tx_0, rx_0) = channel();
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();
        let (tx_3, rx_3) = channel();
        let (tx_4, rx_4) = channel();
        let (tx_ans, rx_ans) = channel();

        tx_0.send(inputs[0]).expect("Cannot init tx_0");
        tx_1.send(inputs[1]).expect("Cannot init tx_1");
        tx_2.send(inputs[2]).expect("Cannot init tx_2");
        tx_3.send(inputs[3]).expect("Cannot init tx_3");
        tx_4.send(inputs[4]).expect("Cannot init tx_4");

        tx_0.send(0).expect("Cannot feed first input");

        let m_0 = memory.clone();
        let t_0 = thread::Builder::new().name("0".to_string()).spawn(move || {
            let mut c_0 = computer::new(m_0.len(), &m_0);
            c_0.set_reader(Box::new(reader::channel(rx_0)));
            c_0.register_writer(Box::new(writer::channel(tx_1)));
            c_0.execute();
        });
        let m_1 = memory.clone();
        let t_1 = thread::Builder::new().name("1".to_string()).spawn(move || {
            let mut c_1 = computer::new(m_1.len(), &m_1);
            c_1.set_reader(Box::new(reader::channel(rx_1)));
            c_1.register_writer(Box::new(writer::channel(tx_2)));
            c_1.execute();
        });
        let m_2 = memory.clone();
        let t_2 = thread::Builder::new().name("2".to_string()).spawn(move || {
            let mut c_2 = computer::new(m_2.len(), &m_2);
            c_2.set_reader(Box::new(reader::channel(rx_2)));
            c_2.register_writer(Box::new(writer::channel(tx_3)));
            c_2.execute();
        });
        let m_3 = memory.clone();
        let t_3 = thread::Builder::new().name("3".to_string()).spawn(move || {
            let mut c_3 = computer::new(m_3.len(), &m_3);
            c_3.set_reader(Box::new(reader::channel(rx_3)));
            c_3.register_writer(Box::new(writer::channel(tx_4)));
            c_3.execute();
        });
        let m_4 = memory.clone();
        let t_4 = thread::Builder::new().name("4".to_string()).spawn(move || {
            let mut c_4 = computer::new(m_4.len(), &m_4);
            c_4.set_reader(Box::new(reader::channel(rx_4)));
            c_4.register_writer(Box::new(writer::channel(tx_0)));
            c_4.register_writer(Box::new(writer::channel(tx_ans)));
            c_4.execute();
        });

        t_0.unwrap().join().expect("t_0");
        t_1.unwrap().join().expect("t_0");
        t_2.unwrap().join().expect("t_0");
        t_3.unwrap().join().expect("t_0");
        t_4.unwrap().join().expect("t_0");

        loop {
            let o = rx_ans.recv_timeout(Duration::from_millis(10)).unwrap_or(-1);
            if o == -1 {
                break;
            }
            if output < o {
                output = o;
            }
        }
    }

    println!("Part 2: {}", output);
}

fn get_permutations(offset: i64) -> Vec<Vec<i64>> {
    let mut permutations = Vec::new();
    for i0 in offset..offset + 5 {
        for i1 in offset..offset + 5 {
            if i0 == i1 {
                continue;
            }
            for i2 in offset..offset + 5 {
                if i0 == i2 || i1 == i2 {
                    continue;
                }
                for i3 in offset..offset + 5 {
                    if i0 == i3 || i1 == i3 || i2 == i3 {
                        continue;
                    }
                    for i4 in offset..offset + 5 {
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
