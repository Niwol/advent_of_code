use std::num::Wrapping;

fn pad(bytes: &mut Vec<u8>) {
    let original_lenght = bytes.len() * 8;
    bytes.push(0x80);

    while bytes.len() % 64 != 56 {
        bytes.push(0x00);
    }

    for i in 0..8 {
        let val = (original_lenght >> (56 - i * 8)) as u8;
        bytes.push(val);
    }
}

fn u8_to_u32(chunk: &[u8; 4]) -> u32 {
    let mut res = 0;
    for i in 0..4 {
        res <<= 8;
        res += chunk[i] as u32;
    }

    res
}

fn make_chunk(bytes: &Vec<u8>, chunk_idx: usize) -> [u32; 16] {
    let mut chunk = [0; 16];
    for j in 0..16 {
        let mut byte_slice = [0; 4];
        for k in 0..4 {
            byte_slice[k] = bytes[chunk_idx * 64 + j * 4 + k];
        }
        chunk[j] = u8_to_u32(&byte_slice);
    }

    chunk
}

fn left_rotate(x: u32, amount: usize) -> u32 {
    x << amount | x >> (32 - amount)
}

fn process_bytes(bytes: &Vec<u8>) -> u128 {
    let mut a = 0x67452301 as u32;
    let mut b = 0xefcdab89 as u32;
    let mut c = 0x98badcfe as u32;
    let mut d = 0x10325476 as u32;

    #[rustfmt::skip]
    let k = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    #[rustfmt::skip]
    let s = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    for offset in (0..bytes.len()).step_by(64) {
        let chunk = make_chunk(&bytes, offset);

        for i in 0..64 {
            let mut f = 0;
            let mut g = 0;

            if i <= 15 {
                f = (b & c) | (!b & d);
                g = i;
            } else if 16 <= i && i <= 31 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) % 16;
            } else if 32 <= i && i <= 47 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if 48 <= i && i <= 63 {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            }

            let mut x = Wrapping(f);
            x = x + Wrapping(a);
            x = x + Wrapping(k[i]);
            x = x + Wrapping(chunk[g]);

            //f = f + a + k[i] + chunk[g];
            f = x.0;
            a = d;
            d = c;
            c = b;

            b = (Wrapping(b) + Wrapping(left_rotate(f, s[i]))).0;
        }
    }

    let mut res = 0;

    for val in [a, b, c, d] {
        res <<= 32;
        res += val as u128;
    }

    res
}

fn md5(input: &str) -> u128 {
    let mut bytes = Vec::new();
    input.as_bytes().iter().for_each(|b| {
        bytes.push(*b);
    });

    pad(&mut bytes);

    for val in bytes.iter() {
        print!("{:2x} ", val);
    }
    println!();

    process_bytes(&bytes)
}

fn v1() {
    let data = "The quick brown fox jumps over the lazy dog";

    let result = md5(data);

    println!("Input string: \"{}\"", data);
    println!("md5: {:32x}", result);
}

fn main() {
    v1();
}
