pub trait Reader {
    fn read(&mut self) -> i64;
}

pub struct StdIn {}

impl Reader for StdIn {
    fn read(&mut self) -> i64 {
        use std::io::{stdin, stdout, Write};

        print!("Please enter a number: ");
        stdout().flush();

        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Input is not correct.");
        if let Some('\n') = buffer.chars().next_back() {
            buffer.pop();
        }
        if let Some('\r') = buffer.chars().next_back() {
            buffer.pop();
        }
        return buffer.parse().unwrap();
    }
}

pub struct Once {
    value: i64,
    has_sent: bool,
}

impl Reader for Once {
    fn read(&mut self) -> i64 {
        if self.has_sent {
            panic!("Can only send once.");
        }
        self.has_sent = true;
        return self.value;
    }
}

pub fn once(value: i64) -> Once {
    return Once {
        value: value,
        has_sent: false,
    };
}
