extern crate clap;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use clap::{App, Arg};
mod days;
mod computer;
mod reader;
mod writer;

fn main() {
    let (day, input) = get_args();
    match day {
        1 => days::day_1::run(input.as_ref()),
        2 => days::day_2::run(input.as_ref()),
        3 => days::day_3::run(input.as_ref()),
        4 => days::day_4::run(),
        5 => days::day_5::run(input.as_ref()),
        7 => days::day_7::run(input.as_ref()),
        _ => panic!("Cannot find wanted day."),
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
                .possible_values(&["1", "2", "3", "4", "5", "7"]),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .required(false),
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap().parse().unwrap();
    let input: String = String::from(matches.value_of("input").unwrap_or(""));
    return (day, input);
}
