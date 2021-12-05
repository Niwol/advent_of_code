use std::{collections::HashMap, fmt::Display, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut covered_points: HashMap<Point, i32> = HashMap::new();

    data.split("\n")
        .map(|s| parse_into_segment(s))
        .for_each(|v| {
            for &p in v.iter() {
                if covered_points.contains_key(&p) == false {
                    covered_points.insert(p, 0);
                }

                *covered_points.get_mut(&p).unwrap() += 1;
            }
        });

    let count = covered_points.iter().filter(|(_, n)| **n > 1).count();

    println!("count = {}", count);

    println!("{:?}", parse_into_segment("2,5 -> 5,2"));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn segment(p1: Point, p2: Point) -> Vec<Point> {
    let mut segment = Vec::new();
    let dist = i32::max(i32::abs(p1.x - p2.x), i32::abs(p1.y - p2.y));
    let mut dx = p2.x - p1.x;
    let mut dy = p2.y - p1.y;

    dx = dx.clamp(-1, 1);
    dy = dy.clamp(-1, 1);

    // V1: if dx == 0 || dy == 0
    for i in 0..=dist {
        segment.push(Point {
            x: p1.x + i * dx,
            y: p1.y + i * dy,
        })
    }

    segment
}

fn parse_into_point(s: &str) -> Point {
    let x;
    let y;
    let mut split = s.split(",");
    x = split.next().unwrap().parse().unwrap();
    y = split.next().unwrap().parse().unwrap();

    Point { x, y }
}

fn parse_into_segment(s: &str) -> Vec<Point> {
    let p1;
    let p2;

    let mut split = s.split(" -> ");
    p1 = parse_into_point(split.next().unwrap());
    p2 = parse_into_point(split.next().unwrap());

    segment(p1, p2)
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}, {} }}", self.x, self.y)
    }
}
