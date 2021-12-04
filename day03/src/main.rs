use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    // V1
    let numbers = data
        .split_ascii_whitespace()
        .map(|s| {
            let mut n = 0;
            s.chars().for_each(|c| {
                if let Some(v) = c.to_digit(10) {
                    n = n << 1;
                    n += v;
                }
            });
            n
        })
        .collect::<Vec<u32>>();

    let (ones, zeros) = count_zo(&numbers);

    let mut gama = 0;
    let mut epsilon = 0;

    for i in 0..12 {
        let idx = 11 - i;
        if ones[idx] > zeros[idx] {
            gama = gama << 1;
            gama += 1;

            epsilon = epsilon << 1;
        } else {
            epsilon = epsilon << 1;
            epsilon += 1;

            gama = gama << 1;
        }
    }

    println!("V1");
    println!("res = {}", gama * epsilon);

    // V2

    // Oxygen
    let mut oxy_numbers = numbers.clone();
    let mut bit_idx = 11;

    while oxy_numbers.len() > 1 {
        let (o, z) = count_zo(&oxy_numbers);

        oxy_numbers = oxy_numbers
            .into_iter()
            .filter(|n| {
                let most_significant = if o[bit_idx as usize] >= z[bit_idx as usize] {
                    1
                } else {
                    0
                };
                (n >> bit_idx) & 1 == most_significant
            })
            .collect::<Vec<u32>>();

        bit_idx -= 1;
    }
    let oxy = oxy_numbers[0];

    // CO2
    let mut co2_numbers = numbers.clone();
    let mut bit_idx = 11;

    while co2_numbers.len() > 1 && bit_idx >= 0 {
        let (o, z) = count_zo(&co2_numbers);

        co2_numbers = co2_numbers
            .into_iter()
            .filter(|n| {
                let least_significant = if o[bit_idx as usize] < z[bit_idx as usize] {
                    1
                } else {
                    0
                };
                (n >> bit_idx) & 1 == least_significant
            })
            .collect::<Vec<u32>>();

        bit_idx -= 1;
    }
    let co2 = co2_numbers[0];

    println!("V2");
    println!("res = {}", oxy * co2);
}

fn count_zo(numbers: &Vec<u32>) -> ([i32; 12], [i32; 12]) {
    let mut ones = [0; 12];
    let mut zeros = [0; 12];

    numbers.iter().for_each(|n| {
        for i in 0..12 {
            if n & 1 << i > 0 {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
        }
    });

    (ones, zeros)
}
