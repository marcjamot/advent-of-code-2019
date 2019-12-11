use std::sync::mpsc::Sender;

pub trait Writer {
    fn write(&self, value: i64);
}

pub struct StdOut {}

impl Writer for StdOut {
    fn write(&self, value: i64) {
        use std::io::{stdout, Write};
        println!("Log: {}", value);
        stdout().flush();
    }
}

pub struct Channel {
    sender: Sender<i64>,
}

impl Writer for Channel {
    fn write(&self, value: i64) {
        self.sender.send(value);
    }
}

pub fn channel(sender: Sender<i64>) -> Channel {
    return Channel { sender: sender };
}
