use crate::Direction::*;
use std::fs;
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut x = 0;
    let mut y = 0;

    data.split('\n')
        .map(|s| {
            let command = s.split(' ').next().unwrap();
            let val = s.split(' ').skip(1).next().unwrap().parse::<i32>().unwrap();
            println!("{}", command);

            match command {
                "forward" => Forward(val),
                "up" => Up(val),
                "down" => Down(val),
                _ => panic!("Wrog command"),
            }
        })
        .for_each(|command| {
            match command {
                Forward(v) => x += v,
                Up(v) => y -= v,
                Down(v) => y += v,
            };
        });

    println!("V1: {}", x * y);
}
