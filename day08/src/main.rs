use std::{collections::HashMap, fs};

//  ------               ------     ------                 ------     ------     ------     ------     ------
// |      |        |           |          |    |      |   |          |                 |   |      |   |      |
// |      |        |           |          |    |      |   |          |                 |   |      |   |      |
// -      -        -     ------     ------      ------     ------     ------           -    ------     ------
// |      |        |    |                 |           |          |   |      |          |   |      |          |
// |      |        |    |                 |           |          |   |      |          |   |      |          |
//  ------               ------     ------                 ------     ------                ------     ------
//
//    6         2          5           5          4           5         6          3          7           6

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let displays = data.split('\n').collect::<Vec<&str>>();

    let mut count = 0;

    for d in displays.iter() {
        let output = d.split('|').map(|s| s.to_string()).skip(1).next().unwrap();
        let output = output.split_ascii_whitespace().collect::<Vec<&str>>();

        for o in output {
            match o.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            };
        }
    }

    println!("V1\ncount = {}", count);

    let mut res = 0;

    for display in displays.iter() {
        let inputs = display
            .split("|")
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        let outputs = display
            .split("|")
            .skip(1)
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();

        let signals = get_signals(inputs);

        res += decode(signals, outputs);
    }

    println!("V2\nres = {}", res);
}

#[derive(Default, Debug)]
struct Signals {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
}

fn get_signals(inputs: Vec<&str>) -> Signals {
    let mut str_to_digit: HashMap<&str, i32> = HashMap::new();
    let mut zero_69 = Vec::new();
    let mut two_35 = Vec::new();
    inputs.clone().into_iter().for_each(|s| match s.len() {
        2 => {
            str_to_digit.insert(s, 1);
        }
        3 => {
            str_to_digit.insert(s, 7);
        }
        4 => {
            str_to_digit.insert(s, 4);
        }
        7 => {
            str_to_digit.insert(s, 8);
        }
        6 => {
            zero_69.push(s);
        }
        5 => {
            two_35.push(s);
        }
        _ => (),
    });

    let mut digit_to_str = HashMap::new();
    for (k, v) in str_to_digit.iter() {
        digit_to_str.insert(v, k);
    }

    let mut sigs = Signals {
        ..Default::default()
    };

    // a_sig
    for c in digit_to_str.get(&7).unwrap().chars() {
        if digit_to_str.get(&1).unwrap().contains(c) == false {
            sigs.a = c;
        }
    }

    // c_sig
    for &s in zero_69.iter() {
        for c in digit_to_str.get(&1).unwrap().chars() {
            if !s.contains(c) {
                sigs.c = c;
            }
        }
    }

    // f_sig
    for c in digit_to_str.get(&1).unwrap().chars() {
        if c != sigs.c {
            sigs.f = c;
        }
    }

    // d_sig
    for &s in zero_69.iter() {
        for c in digit_to_str.get(&4).unwrap().chars() {
            if c != sigs.c && !s.contains(c) {
                sigs.d = c;
            }
        }
    }

    // b_sig
    for c in digit_to_str.get(&4).unwrap().chars() {
        if c != sigs.c && c != sigs.d && c != sigs.f {
            sigs.b = c;
        }
    }

    // g_sig
    for &s in two_35.iter() {
        if !s.contains(sigs.c) {
            for c in s.chars() {
                if c != sigs.a && c != sigs.b && c != sigs.d && c != sigs.f {
                    sigs.g = c;
                }
            }
        }
    }

    // e_sig
    for c in digit_to_str.get(&8).unwrap().chars() {
        if c != sigs.a && c != sigs.b && c != sigs.c && c != sigs.d && c != sigs.f && c != sigs.g {
            sigs.e = c;
        }
    }

    // println!(
    //     "{:?}    a: {}   b: {}   c: {}   d: {}   e: {}   f: {},   g: {}",
    //     digit_to_str, sigs.a, sigs.b, sigs.c, sigs.d, sigs.e, sigs.f, sigs.g
    // );

    sigs
}

fn decode(signals: Signals, output: Vec<&str>) -> i32 {
    let mut res = 0;

    for &s in output.iter() {
        // 0
        if s.contains(signals.a)
            && s.contains(signals.b)
            && s.contains(signals.c)
            && !s.contains(signals.d)
            && s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 0;
        }

        // 1
        if !s.contains(signals.a)
            && !s.contains(signals.b)
            && s.contains(signals.c)
            && !s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && !s.contains(signals.g)
        {
            res *= 10;
            res += 1;
        }

        // 2
        if s.contains(signals.a)
            && !s.contains(signals.b)
            && s.contains(signals.c)
            && s.contains(signals.d)
            && s.contains(signals.e)
            && !s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 2;
        }

        // 3
        if s.contains(signals.a)
            && !s.contains(signals.b)
            && s.contains(signals.c)
            && s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 3;
        }

        // 4
        if !s.contains(signals.a)
            && s.contains(signals.b)
            && s.contains(signals.c)
            && s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && !s.contains(signals.g)
        {
            res *= 10;
            res += 4;
        }

        // 5
        if s.contains(signals.a)
            && s.contains(signals.b)
            && !s.contains(signals.c)
            && s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 5;
        }

        // 6
        if s.contains(signals.a)
            && s.contains(signals.b)
            && !s.contains(signals.c)
            && s.contains(signals.d)
            && s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 6;
        }

        // 7
        if s.contains(signals.a)
            && !s.contains(signals.b)
            && s.contains(signals.c)
            && !s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && !s.contains(signals.g)
        {
            res *= 10;
            res += 7;
        }

        // 8
        if s.contains(signals.a)
            && s.contains(signals.b)
            && s.contains(signals.c)
            && s.contains(signals.d)
            && s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 8;
        }

        // 9
        if s.contains(signals.a)
            && s.contains(signals.b)
            && s.contains(signals.c)
            && s.contains(signals.d)
            && !s.contains(signals.e)
            && s.contains(signals.f)
            && s.contains(signals.g)
        {
            res *= 10;
            res += 9;
        }
    }

    res
}
