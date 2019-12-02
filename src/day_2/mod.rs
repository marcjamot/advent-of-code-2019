use std::fs;

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    part_1(&inputs);
    part_2(&inputs);
}

fn load_inputs(file_name: &str) -> Vec<usize> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
}

fn part_1(inputs: &Vec<usize>) {
    let mut instructions = inputs.clone();
    instructions[1] = 12;
    instructions[2] = 2;
    execute(&mut instructions);
    println!(
        "Part 1: [{}]",
        instructions
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn part_2(inputs: &Vec<usize>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions = inputs.clone();
            instructions[1] = noun;
            instructions[2] = verb;
            if !execute(&mut instructions) {
                continue;
            }
            if instructions[0] == 19690720 {
                println!(
                    "Part 2: Noun [{}] Verb [{}] -> [{}]",
                    noun,
                    verb,
                    instructions
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
                return;
            }
        }
    }
    println!("No answer found for part 2 :(");
}

fn execute(instructions: &mut Vec<usize>) -> bool {
    let mut pointer = 0;
    while instructions[pointer] != 99 {
        let a = instructions[pointer + 1];
        let b = instructions[pointer + 2];
        let r = instructions[pointer + 3];
        match instructions[pointer] {
            1 => {
                instructions[r] = instructions[a] + instructions[b];
            }
            2 => {
                instructions[r] = instructions[a] * instructions[b];
            }
            _ => return false,
        }
        pointer += 4;
    }
    return true;
}
