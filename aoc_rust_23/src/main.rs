// use std::fs;
pub mod aoc_reader {
    use std::fs;
    fn number_to_string(day_n: u8) -> String {
        let mut d = day_n;
        let mut str_day = String::new();
        while d != 0 {
            let n = d % 10;
            d = d / 10;
            str_day.insert(0, (n + ('0' as u8)) as char);
        }

        return str_day;
    }

    pub fn read_day_input(day_number: u8) -> String {
        let input_location =
            "E:\\dev\\aoc23\\input\\d".to_owned() + &number_to_string(day_number) + ".txt";
        match fs::read_to_string(input_location) {
            Ok(res) => return res,
            _ => {
                println!("ERROR NO FILE FOUND");
                return "".to_string();
            }
        }
    }
}

pub mod day;
use day::{d1, d2, d3, d4, d5};
fn main() {
    day::d5::part2();
}
