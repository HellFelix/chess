#![allow(dead_code)]

use core::panic;

// integer representations of squares to be used by lookup functions
pub const a1: i32 = 0;
pub const b1: i32 = 1;
pub const c1: i32 = 2;
pub const d1: i32 = 3;
pub const e1: i32 = 4;
pub const f1: i32 = 5;
pub const g1: i32 = 6;
pub const h1: i32 = 7;
pub const a2: i32 = 8;
pub const b2: i32 = 9;
pub const c2: i32 = 10;
pub const d2: i32 = 11;
pub const e2: i32 = 12;
pub const f2: i32 = 13;
pub const g2: i32 = 14;
pub const h2: i32 = 15;
pub const a3: i32 = 16;
pub const b3: i32 = 17;
pub const c3: i32 = 18;
pub const d3: i32 = 19;
pub const e3: i32 = 20;
pub const f3: i32 = 21;
pub const g3: i32 = 22;
pub const h3: i32 = 23;
pub const a4: i32 = 24;
pub const b4: i32 = 25;
pub const c4: i32 = 26;
pub const d4: i32 = 27;
pub const e4: i32 = 28;
pub const f4: i32 = 29;
pub const g4: i32 = 30;
pub const h4: i32 = 31;
pub const a5: i32 = 32;
pub const b5: i32 = 33;
pub const c5: i32 = 34;
pub const d5: i32 = 35;
pub const e5: i32 = 36;
pub const f5: i32 = 37;
pub const g5: i32 = 38;
pub const h5: i32 = 39;
pub const a6: i32 = 40;
pub const b6: i32 = 41;
pub const c6: i32 = 42;
pub const d6: i32 = 43;
pub const e6: i32 = 44;
pub const f6: i32 = 45;
pub const g6: i32 = 46;
pub const h6: i32 = 47;
pub const a7: i32 = 48;
pub const b7: i32 = 49;
pub const c7: i32 = 50;
pub const d7: i32 = 51;
pub const e7: i32 = 52;
pub const f7: i32 = 53;
pub const g7: i32 = 54;
pub const h7: i32 = 55;
pub const a8: i32 = 56;
pub const b8: i32 = 57;
pub const c8: i32 = 58;
pub const d8: i32 = 59;
pub const e8: i32 = 60;
pub const f8: i32 = 61;
pub const g8: i32 = 62;
pub const h8: i32 = 63;

pub fn from_str(s: &str) -> i32 {
    let mut c = s.chars();
    let mut res = 0;
    match c.next().unwrap() {
        'a' => res += 0,
        'b' => res += 1,
        'c' => res += 2,
        'd' => res += 3,
        'e' => res += 4,
        'f' => res += 5,
        'g' => res += 6,
        'h' => res += 7,
        '-' => return -1,
        _ => {}
    }
    res += (c.next().unwrap().to_string().parse::<i32>().unwrap() - 1) * 8;
    res
}

pub fn to_str(n: i32) -> Option<String> {
    if n >= 0 && n < 64 {
        let mut res = String::from(match n % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid square number"),
        });

        res += &format!("{}", n / 8 + 1);

        Some(res)
    } else {
        None
    }
}
