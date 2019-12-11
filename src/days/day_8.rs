use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

pub fn run(input: &str) {
    let pixels = load_inputs(input);
    part_1(&pixels);
}

fn load_inputs(file_name: &str) -> Vec<u32> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content.chars().map(|x| x.to_digit(10).unwrap()).collect();
}

fn part_1(pixels: &Vec<u32>) {
    let layers = pixels.len() / LAYER_SIZE;
    assert_eq!(pixels.len(), layers * LAYER_SIZE);
    println!("Part 1 has {} layers", layers);
    let mut layer_low_zeros: usize = std::usize::MAX;
    let mut low_n_zeros: usize = std::usize::MAX;
    for layer in 0..layers {
        let mut n_zeros: usize = 0;
        for i in layer * LAYER_SIZE..(layer + 1) * LAYER_SIZE {
            if pixels[i] == 0 {
                n_zeros += 1;
            }
        }
        if n_zeros < low_n_zeros {
            layer_low_zeros = layer;
            low_n_zeros = n_zeros;
        }
    }
    println!("Lowest layer: {}", layer_low_zeros);

    let mut n_ones = 0;
    let mut n_twos = 0;
    for i in layer_low_zeros * LAYER_SIZE..(layer_low_zeros + 1) * LAYER_SIZE {
        if pixels[i] == 1 {
            n_ones += 1;
        }
        if pixels[i] == 2 {
            n_twos += 1;
        }
    }
    println!(
        "Ones: {}, Twos: {}, ones*twos: {}",
        n_ones,
        n_twos,
        n_ones * n_twos
    );
}

fn part_2(pixels: &Vec<u32>) {
}
