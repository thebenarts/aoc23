use std::fs;

use fs::File;
fn ViewToString(view: &str) -> Option<i32> {
    let first_char = view.as_bytes()[0] as char;
    if first_char.is_ascii_digit() {
        let num: i32 = first_char as i32 & 0x0F;
        return Some(num);
    }

    let mut num;
    if view.len() >= 3 {
        let three_view = &view[..3];
        num = match three_view {
            "one" => Some(1),
            "two" => Some(2),
            "six" => Some(6),
            _ => None,
        };

        match num {
            Some(x) => return num,
            None => (),
        }
    }

    if view.len() >= 4 {
        let four_view = &view[..4];
        num = match four_view {
            "four" => Some(4),
            "five" => Some(5),
            "nine" => Some(9),
            _ => None,
        };

        match num {
            Some(x) => return num,
            None => (),
        }
    }

    if view.len() >= 5 {
        let five_view = &view[..5];
        num = match five_view {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None,
        };

        match num {
            Some(x) => return num,
            None => (),
        }
    }

    return None;
}

fn main() {
    let input = fs::read_to_string("E:\\dev\\aoc23\\input\\d1.txt");
    let mut result = 0;
    let mut nums: [u8; 2] = [0; 2];

    for line in input.unwrap().lines() {
        for number in 0..line.len() {
            let view = &line[number..];
            if let Some(res) = ViewToString(view) {
                nums[0] = res as u8;
                break;
            }
        }

        for number in (0..line.len()).rev() {
            let view = &line[number..];
            if let Some(res) = ViewToString(view) {
                nums[1] = res as u8;
                break;
            }
        }
        println!("{}  --- [0] {}   [1] {}", line, &nums[0], &nums[1]);

        result += nums[0] as u32 * 10 + nums[1] as u32;
    }

    println!("{}", result);
}
