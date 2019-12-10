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
