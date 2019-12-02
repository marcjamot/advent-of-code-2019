extern crate clap;
use clap::{App, Arg};
mod day_1;

fn main() {
    let (day, input) = get_args();
    match day {
        1 => day_1::run(input.as_ref()),
        _ => panic!("ASF"),
    }
}

fn get_args() -> (u8, String) {
    let matches = App::new("Advent of Code 2019")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("NUMBER")
                .help("Day to execute, 1-25")
                .required(true)
                .possible_values(&["1", "2"]),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .required(true),
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap().parse().unwrap();
    let input: String = String::from(matches.value_of("input").unwrap());
    return (day, input);
}
