use std::fs;

pub fn run() {
    let inputs = load_inputs("inputs/day_1.txt");
    part_1(&inputs);
    part_2(&inputs);
}

fn load_inputs(file_name: &str) -> Vec<f64> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content.lines().map(|x| x.parse::<f64>().unwrap()).collect();
}

fn fuel_calculation(mass: &f64) -> f64 {
    return (mass / 3.0).floor() - 2.0;
}

fn part_1(inputs: &Vec<f64>) {
    let fuel: f64 = inputs.iter().map(fuel_calculation).sum();
    println!("Part 1: Fuel mass {}", fuel);
}

fn part_2(inputs: &Vec<f64>) {
    let fuel: f64 = inputs
        .iter()
        .map(fuel_calculation)
        .map(|x| x + additional_fuel_calculation(x))
        .sum();
    println!("Part 2: Fuel mass {}", fuel);
}

fn additional_fuel_calculation(fuel_weight: f64) -> f64 {
    if fuel_weight == 0.0 {
        return 0.0;
    }

    let mut additional_fuel = fuel_calculation(&fuel_weight);
    if additional_fuel < 0.0 {
        additional_fuel = 0.0;
    }
    return additional_fuel + additional_fuel_calculation(additional_fuel);
}
