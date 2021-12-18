use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut map = Vec::new();

    data.split('\n').for_each(|s| {
        map.push(Vec::new());
        for c in s.chars() {
            let line = map.last_mut().unwrap();
            line.push((c.to_digit(10).unwrap(), false));
        }
    });

    let mut local_mins = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut min = true;
            if i > 0 && map[i][j] >= map[i - 1][j] {
                min = false;
            }
            if i < map.len() - 1 && map[i][j] >= map[i + 1][j] {
                min = false;
            }
            if j > 0 && map[i][j] >= map[i][j - 1] {
                min = false;
            }
            if j < map[0].len() - 1 && map[i][j] >= map[i][j + 1] {
                min = false;
            }

            if min {
                local_mins.push((i, j));
            }
        }
    }

    let mut res: u32 = 0;
    for &(i, j) in local_mins.iter() {
        res += map[i][j].0 + 1;
    }
    println!("V1\nres = {}", res);

    let mut bassins = Vec::new();
    for (i, &(x, y)) in local_mins.iter().enumerate() {
        bassins.push(0);
        let mut list = vec![(x, y)];
        map[x][y].1 = true;

        while !list.is_empty() {
            let (x, y) = list.pop().unwrap();

            // mark
            bassins[i] += 1;

            // next neighbors
            if x > 0 && !map[x - 1][y].1 && map[x - 1][y].0 != 9 {
                map[x - 1][y].1 = true;
                list.push((x - 1, y));
            }

            if x < map.len() - 1 && !map[x + 1][y].1 && map[x + 1][y].0 != 9 {
                map[x + 1][y].1 = true;
                list.push((x + 1, y));
            }

            if y > 0 && !map[x][y - 1].1 && map[x][y - 1].0 != 9 {
                map[x][y - 1].1 = true;
                list.push((x, y - 1));
            }

            if y < map[0].len() - 1 && !map[x][y + 1].1 && map[x][y + 1].0 != 9 {
                map[x][y + 1].1 = true;
                list.push((x, y + 1));
            }
        }
        // println!("{{ {}, {} }}: {}  {}", x, y, map[x][y].0, bassins[i]);
    }

    bassins.sort_by(|x, y| y.partial_cmp(x).unwrap());

    let res = bassins[0] * bassins[1] * bassins[2];
    println!("V2\nres = {}", res);
}
