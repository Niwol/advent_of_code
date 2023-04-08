use std::collections::HashMap;
use std::fs;

fn v1() {
    let data = fs::read_to_string("day03/data.txt").unwrap();

    let mut current_coord = (0, 0);
    let mut map = HashMap::new();

    map.insert(current_coord, 1);

    data.chars().for_each(|c| {
        match c {
            '>' => current_coord.0 += 1,
            '<' => current_coord.0 -= 1,
            '^' => current_coord.1 += 1,
            'v' => current_coord.1 -= 1,
            _ => (),
        };

        match map.get_mut(&current_coord) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(current_coord, 1);
            }
        };
    });

    println!("Houses visitied: {}", map.len());
}

fn v2() {
    let data = fs::read_to_string("day03/data.txt").unwrap();

    let mut n = 0;

    let mut santa_coord = (0, 0);
    let mut robot_coord = (0, 0);
    let mut map = HashMap::new();

    map.insert(santa_coord, 2);

    data.chars().for_each(|c| {
        let mut current_mover = if n % 2 == 0 {
            &mut santa_coord
        } else {
            &mut robot_coord
        };

        match c {
            '>' => current_mover.0 += 1,
            '<' => current_mover.0 -= 1,
            '^' => current_mover.1 += 1,
            'v' => current_mover.1 -= 1,
            _ => (),
        };

        match map.get_mut(&current_mover) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(*current_mover, 1);
            }
        };

        n += 1;
    });

    println!("Houses visitied V2: {}", map.len());
}

fn main() {
    v1();
    v2();
}
