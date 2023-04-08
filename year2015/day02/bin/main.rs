use std::fs;

fn v1() {
    let data = fs::read_to_string("day02/data.txt").unwrap();

    let mut total_area = 0;

    data.split('\n').for_each(|s| {
        if s.is_empty() {
            return;
        }

        let mut dimensions = s.split('x').map(|s| s.parse::<u32>().unwrap());

        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();
        let l = dimensions.next().unwrap();

        let side_1 = w * h;
        let side_2 = w * l;
        let side_3 = h * l;

        let smalest_side = *[side_1, side_2, side_3].iter().min().unwrap();

        total_area += 2 * side_1 + 2 * side_2 + 2 * side_3 + smalest_side;
    });

    println!("Total area: {}", total_area);
}

fn v2() {
    let data = fs::read_to_string("day02/data.txt").unwrap();

    let mut total_lenght = 0;

    data.split('\n').for_each(|s| {
        if s.is_empty() {
            return;
        }

        let mut dimensions = s.split('x').map(|s| s.parse::<u32>().unwrap());

        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();
        let l = dimensions.next().unwrap();

        let perim_1 = 2 * w + 2 * h;
        let perim_2 = 2 * w + 2 * l;
        let perim_3 = 2 * h + 2 * l;

        let smalest_perim = *[perim_1, perim_2, perim_3].iter().min().unwrap();

        let volume = w * h * l;

        total_lenght += smalest_perim + volume;
    });

    println!("Total lenght: {}", total_lenght);
}

fn main() {
    v1();
    v2();
}
