use num::integer::gcd;
use std::collections::{BTreeMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
}

struct Map {
    stars: Vec<Star>,
    collisions: HashSet<(i32, i32)>,
}

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    let (x, y) = part_1(&inputs);
    part_2(&inputs, x, y);
}

fn load_inputs(file_name: &str) -> Map {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    let rows: Vec<&str> = content.lines().collect();
    let mut stars = Vec::new();
    let mut collisions = HashSet::new();
    for (y, row) in rows.iter().enumerate() {
        let cols: Vec<char> = row.chars().collect();
        for (x, c) in cols.iter().enumerate() {
            if *c == '#' {
                stars.push(Star {
                    x: x as i32,
                    y: y as i32,
                });
                collisions.insert((x as i32, y as i32));
            }
        }
    }

    assert_eq!(stars.len(), collisions.len());
    let width = rows[0].len() as u8;
    let height = rows.len() as u8;
    println!("Map w {} x h {} with {} stars", width, height, stars.len());
    return Map {
        stars: stars,
        collisions: collisions,
    };
}

fn part_1(map: &Map) -> (i32, i32) {
    let mut best_visible_stars = 0;
    let mut best_position = -1;
    for (i, star) in (&map.stars).iter().enumerate() {
        let mut visible_stars = 0;
        for (j, other) in (&map.stars).iter().enumerate() {
            if i == j {
                continue;
            }

            if is_visible(&map.collisions, star.x, star.y, other) {
                visible_stars += 1;
            }
        }
        if best_visible_stars < visible_stars {
            best_visible_stars = visible_stars;
            best_position = i as i32;
        }
    }
    let best_star = &map.stars[best_position as usize];
    println!(
        "Best star with {} visible is {:?}",
        best_visible_stars, best_star
    );
    return (best_star.x, best_star.y);
}

fn part_2(map: &Map, x: i32, y: i32) {
    let mut stars = BTreeMap::new();
    for star in &map.stars {
        if star.x == x && star.y == y {
            continue;
        }

        let dx = star.x as f64 - x as f64;
        let dy = star.y as f64 - y as f64;
        let d = dy.atan2(dx) + std::f64::consts::FRAC_PI_2;
        // if star.x == 11 && star.y == 12 {
        //     println!("11,12 -> {}", d);
        // }
        stars.insert(
            (
                d.trunc() as i32,
                (d.fract() * 100000000000.0 as f64) as i64,
                star.x,
                star.y,
            ),
            star,
        );
    }
    assert_eq!(map.stars.len(), stars.len() + 1);

    let mut collisions = map.collisions.clone();
    let mut destroyed = 0;
    let mut last_destroyed_trunc: i32 = 0;
    let mut last_destroyed_fract: i64 = 0;
    loop {
        for ((t, f, _, _), star) in stars.iter() {
            if destroyed == 0 && (t < &0 || f < &0) {
                continue;
            }
            if !collisions.contains(&(star.x, star.y)) {
                continue;
            }
            if !is_visible(&collisions, x, y, star) {
                continue;
            }
            if destroyed > 0 && t == &last_destroyed_trunc && f == &last_destroyed_fract {
                continue;
            }
            last_destroyed_trunc = *t;
            last_destroyed_fract = *f;
            collisions.remove(&(star.x, star.y));
            destroyed += 1;
            if destroyed <= 3
                || destroyed == 10
                || destroyed == 20
                || destroyed == 50
                || destroyed == 100
                || destroyed == 199
                || destroyed == 200
            {
                let dx = star.x as f64 - x as f64;
                let dy = star.y as f64 - y as f64;
                let d = dy.atan2(dx) + std::f64::consts::FRAC_PI_2;
                println!("Destroyed {}: {:?} with angle: {}", destroyed, star, d,); // DEBUG
                if destroyed == 200 {
                    return;
                }
            }
        }
    }
}

fn is_visible(
    collisions: &HashSet<(i32, i32)>,
    origin_x: i32,
    origin_y: i32,
    destination: &Star,
) -> bool {
    let mut dx = destination.x - origin_x;
    let mut dy = destination.y - origin_y;
    assert!(dx != 0 || dy != 0);

    if dx == 0 {
        dy = dy.signum();
    } else if dy == 0 {
        dx = dx.signum();
    } else {
        let d = gcd(dx, dy);
        dx /= d;
        dy /= d;
    }

    let mut x = origin_x + dx;
    let mut y = origin_y + dy;
    while x != destination.x || y != destination.y {
        if collisions.contains(&(x, y)) {
            return false;
        }
        x += dx;
        y += dy;
    }

    return true;
}
