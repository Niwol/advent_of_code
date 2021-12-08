use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let positions = data
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut lowest_cost = calc_cost(0, &positions);

    for i in 0..*positions.iter().max().unwrap() {
        let cost = calc_cost(i, &positions);
        if lowest_cost > cost {
            lowest_cost = cost;
        }
    }

    println!("V1\nmin cost: {}", lowest_cost);

    lowest_cost = calc_cost2(0, &positions);

    for i in 0..*positions.iter().max().unwrap() {
        let cost = calc_cost2(i, &positions);
        if lowest_cost > cost {
            lowest_cost = cost;
        }
    }

    println!("V2\nmin cost: {}", lowest_cost);
}

fn calc_cost(pos: i32, vec: &Vec<i32>) -> i32 {
    let mut cost = 0;
    for &i in vec.iter() {
        cost += i32::abs(pos - i);
    }

    cost
}

fn calc_cost2(pos: i32, vec: &Vec<i32>) -> i32 {
    let mut cost = 0;
    for &i in vec.iter() {
        let length = (pos - i).abs();
        cost += ((length) * (length + 1)) / 2;
    }

    cost
}
