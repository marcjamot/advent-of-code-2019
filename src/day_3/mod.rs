use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Path {
    direction: String,
    distance: u32,
}

#[derive(Debug, Copy, Clone)]
struct Step {
    x: i32,
    y: i32,
    steps: u32,
}

pub fn run(input: &str) {
    let (left_paths, right_paths) = load_inputs(input);
    let left_steps = paths_to_steps(&left_paths);
    let right_steps = paths_to_steps(&right_paths);
    part_1(&left_steps, &right_steps);
    part_2(&left_steps, &right_steps);
}

fn load_inputs(file_name: &str) -> (Vec<Path>, Vec<Path>) {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    let both: Vec<&str> = content.lines().collect();
    let left = both[0].split(",").map(string_to_path).collect();
    let right = both[1].split(",").map(string_to_path).collect();
    (left, right)
}

fn string_to_path(path: &str) -> Path {
    let (left, right) = path.split_at(1);
    Path {
        direction: String::from(left),
        distance: right.parse().unwrap(),
    }
}

fn paths_to_steps(paths: &Vec<Path>) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    for path in paths {
        for _ in 0..path.distance {
            match path.direction.as_ref() {
                "U" => y += 1,
                "D" => y -= 1,
                "L" => x -= 1,
                "R" => x += 1,
                _ => panic!("Not valid direction"),
            }
            d += 1;
            steps.push(Step {
                x: x,
                y: y,
                steps: d,
            });
        }
    }

    steps
}

fn part_1(left_steps: &Vec<Step>, right_steps: &Vec<Step>) {
    let mut visited = HashMap::new();

    for left in left_steps {
        let key = format!("{},{}", left.x, left.y);
        visited.insert(key, true);
    }

    let mut distance = std::u32::MAX;
    for right in right_steps {
        let key = format!("{},{}", right.x, right.y);
        let left_visited = *visited.get(&key).unwrap_or(&false);
        if left_visited {
            let d = (right.x.abs() + right.y.abs()) as u32;
            if d < distance {
                distance = d;
            }
        }
    }

    println!("Part 1: {}", distance)
}

fn part_2(left_steps: &Vec<Step>, right_steps: &Vec<Step>) {
    let mut visited = HashMap::new();

    for left in left_steps {
        let key = format!("{},{}", left.x, left.y);
        visited.insert(key, left);
    }

    let mut distance = std::u32::MAX;
    for right in right_steps {
        let key = format!("{},{}", right.x, right.y);
        let left = visited.get(&key);
        if left.is_some() {
            let d = left.unwrap().steps + right.steps;
            if d < distance {
                distance = d;
            }
        }
    }

    println!("Part 2: {}", distance)
}
