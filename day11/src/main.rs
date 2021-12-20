use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut octopuses = Vec::new();
    let mut flashes = 0;

    data.split("\n").for_each(|s| {
        octopuses.push(Vec::new());
        for c in s.chars() {
            octopuses
                .last_mut()
                .unwrap()
                .push((c.to_digit(10).unwrap(), false));
        }
    });

    let mut octopuses_v2 = octopuses.clone();
    print_mat(&octopuses);
    for _ in 0..100 {
        step(&mut octopuses, &mut flashes);
    }

    println!("V1\nres = {}", flashes);

    let mut sync = false;
    let mut steps = 0;
    while !sync {
        step(&mut octopuses_v2, &mut flashes);
        sync = true;
        for v in octopuses_v2.iter() {
            for &(n, _) in v {
                if n != 0 {
                    sync = false;
                }
            }
        }

        steps += 1;
    }

    println!("V2\nres = {}", steps);
}

fn step(octopuses: &mut Vec<Vec<(u32, bool)>>, flashes: &mut i32) {
    // Step 1 | 2
    for i in 0..octopuses.len() {
        for j in 0..octopuses[0].len() {
            octopuses[i][j].0 += 1;

            if octopuses[i][j].0 > 9 && !octopuses[i][j].1 {
                octopuses[i][j].1 = true;
                increase_neight(octopuses, i, j);
            }
        }
    }

    // Step 3
    for i in 0..octopuses.len() {
        for j in 0..octopuses[0].len() {
            if octopuses[i][j].0 > 9 {
                *flashes += 1;
                octopuses[i][j].0 = 0;
                octopuses[i][j].1 = false;
            }
        }
    }
}

fn increase_neight(mat: &mut Vec<Vec<(u32, bool)>>, i: usize, j: usize) {
    let i_low = i32::max(-1, -(i as i32));
    let i_high = i32::min(1, (mat.len() - 1 - i) as i32);
    let j_low = i32::max(-1, -(j as i32));
    let j_high = i32::min(1, (mat.len() - 1 - j) as i32);

    for k in i_low..=i_high {
        for l in j_low..=j_high {
            let x = (i as i32 + k) as usize;
            let y = (j as i32 + l) as usize;
            if !mat[x][y].1 {
                mat[x][y].0 += 1;

                if mat[x][y].0 > 9 {
                    mat[x][y].1 = true;
                    increase_neight(mat, x, y);
                }
            }
        }
    }
}

fn print_mat(mat: &Vec<Vec<(u32, bool)>>) {
    for v in mat.iter() {
        for &n in v.iter() {
            print!("{:2} ", n.0);
        }
        println!();
    }
    println!();
}
