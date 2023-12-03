use crate::aoc_reader;
fn part1() {
    let input = aoc_reader::read_day_input(2);
    let mut result = 0;
    let mut round = Round::new();

    for (i, line) in input.lines().enumerate() {
        let mut view = &line[line.find(':').unwrap() + 2..];
        let mut is_game_valid = true;
        let mut round_split = view.split(';');
        while let Some(round_string) = round_split.next() {
            round.Reset();
            round.ParseRound(round_string);
            // Part1
            is_game_valid = round.IsValid();
            if !is_game_valid {
                break;
            }
            println!("{i}:  {:?}", round);
        }
        if is_game_valid {
            result += i + 1;
        }
    }

    println!("{}", result);
}

fn part2() {
    let input = aoc_reader::read_day_input(2);
    let mut result = 0;
    let mut round = Round::new();
    let mut game_result = Round::new();

    for (i, line) in input.lines().enumerate() {
        let mut view = &line[line.find(':').unwrap() + 2..];
        let mut round_split = view.split(';');
        game_result.Reset();
        while let Some(round_string) = round_split.next() {
            round.Reset();
            round.ParseRound(round_string);
            game_result.Max(&round);
            println!("{i}:  {:?}", round);
        }
        result += game_result.Product();
    }

    println!("{}", result);
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn new() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn ParseRound(&mut self, round_view: &str) {
        let mut color_split = round_view.split(',');
        while let Some(color) = color_split.next() {
            self.ParseColor(color);
        }
    }

    fn ParseColor(&mut self, color_view: &str) {
        let mut split = color_view.split_whitespace();
        let num_str = split.next();
        if let Some(mut color) = split.next() {
            match color {
                "red" => self.red = ToNumber(num_str.unwrap()),
                "green" => self.green = ToNumber(num_str.unwrap()),
                "blue" => self.blue = ToNumber(num_str.unwrap()),
                _ => (),
            }
        }
    }

    fn Reset(&mut self) {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }

    fn IsValid(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }

    fn Max(&mut self, other: &Round) {
        self.red = std::cmp::max(self.red, other.red);
        self.green = std::cmp::max(self.green, other.green);
        self.blue = std::cmp::max(self.blue, other.blue);
    }

    fn Product(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
}

fn ToNumber(view: &str) -> i32 {
    let mut num: i32 = 0;
    let bytes = view.as_bytes();
    for c in bytes.iter() {
        num *= 10;
        if c.is_ascii_digit() {
            num += *c as i32 & 0x0F;
        }
    }

    return num;
}
