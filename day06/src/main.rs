use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let numbers = data
        .split(",")
        .map(|s| (s.parse().unwrap()))
        .collect::<Vec<i32>>();

    let mut fish_tab = [0u64; 9];

    for counter in numbers.iter() {
        fish_tab[*counter as usize] += 1;
    }

    for _ in 0..80 {
        let new_fishes = fish_tab[0];

        for i in 0..8 {
            fish_tab[i] = fish_tab[i + 1];
        }
        fish_tab[6] += new_fishes;
        fish_tab[8] = new_fishes;
    }

    let mut total = 0;
    for i in 0..9 {
        total += fish_tab[i];
    }

    println!("V1\nres = {}", total);

    let mut fish_tab = [0u64; 9];

    for counter in numbers.iter() {
        fish_tab[*counter as usize] += 1;
    }

    for _ in 0..256 {
        let new_fishes = fish_tab[0];

        for i in 0..8 {
            fish_tab[i] = fish_tab[i + 1];
        }
        fish_tab[6] += new_fishes;
        fish_tab[8] = new_fishes;
    }

    let mut total = 0;
    for i in 0..9 {
        total += fish_tab[i];
    }

    println!("V2\nres = {}", total);
}

// 10  2
// 9   1
// 8   0
// 7   6   8
// 6   5   7
// 5   4   6
// 4   3   5
// 3   2   4
// 2   1   3
// 1   0   2   8
// 0   6   1   7
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
