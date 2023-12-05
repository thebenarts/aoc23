use crate::aoc_reader;
use std::collections::HashMap;

const DIRECTIONS: [[i32; 2]; 8] = [
    [0, -1],  // left
    [-1, -1], // left up
    [1, -1],  // left down
    [0, 1],   // right
    [-1, 1],  // right up
    [1, 1],   // right down
    [-1, 0],  // up
    [1, 0],   // down
];

// opposite index so if we find digit at and it goes left we don't want it to move right.
enum MovedInDirection {
    left = 3,
    right = 0,
    up = 7,
    down = 6,
    diagonal = 100,
}

impl MovedInDirection {
    fn get_value(&self) -> usize {
        match self {
            Self::left => 3,
            Self::right => 0,
            _ => 100,
        }
    }

    fn get_direction_from_number(i: usize) -> MovedInDirection {
        match i {
            0 => MovedInDirection::left,
            3 => MovedInDirection::right,
            _ => MovedInDirection::diagonal,
        }
    }
}

fn read_input() -> Vec<Vec<char>> {
    let input = aoc_reader::read_day_input(3);
    let mut full_vec: Vec<Vec<char>> = Vec::new();
    for mut line in input.lines() {
        line = line.trim();
        let bytes = line.as_bytes();
        let mut row: Vec<char> = Vec::new();
        row.reserve(bytes.len());
        for c in bytes.iter() {
            row.push(*c as char);
        }
        full_vec.push(row);
    }

    return full_vec;
}

fn is_index_valid(index: &i32, size: &usize) -> bool {
    return *index >= 0 && index < &(*size as i32);
}

fn validate_index(index: i32, size: &usize) -> Option<i32> {
    if is_index_valid(&index, &size) {
        return Some(index);
    }

    return None;
}

fn check_neighbours_for_symbols(
    input: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    f_dir: MovedInDirection,
    f_n: bool,
) -> bool {
    if is_symbol(&input[row][col]) {
        return true;
    }

    if !f_n {
        return false;
    }
    let mut has_symbol = false;
    if is_number(&input[row][col]) {
        let size = input.len();
        let row: i32 = row as i32;
        let col: i32 = col as i32;

        for (i, mov) in DIRECTIONS.iter().enumerate() {
            let v = validate_index(row + mov[0], &size);
            let h = validate_index(col + mov[1], &size);
            if (v.is_none() || h.is_none()) {
                continue;
            }
            let v = v.unwrap() as usize;
            let h = h.unwrap() as usize;

            if i == f_dir.get_value() {
                continue;
            }

            let can_recurse_in_dir = i == 0 || i == 3;
            has_symbol |= check_neighbours_for_symbols(
                &input,
                v,
                h,
                MovedInDirection::get_direction_from_number(i),
                can_recurse_in_dir,
            );

            if has_symbol {
                return true;
            }
        }
    }
    return has_symbol;
}

fn is_number(element: &char) -> bool {
    return element.is_ascii_digit();
}
fn is_symbol(element: &char) -> bool {
    return !(element.is_ascii_digit() || *element == '.');
}

fn get_valid_number_forward(input: &Vec<Vec<char>>, row: usize, col: &mut usize) -> u32 {
    let size = input.len();
    let mut num: u32 = 0;
    while *col < size && is_number(&input[row][*col]) {
        num *= 10;
        num += input[row][*col] as u32 & 0x0F;
        *col += 1
    }

    if is_index_valid(&(*col as i32), &size) {
        *col -= 1; // If the index is valid we want to step back one so the outer loop can step again and be correct
    }
    return num;
}

pub fn part1() {
    let mut full_vec = read_input();

    let mut result = 0;
    let mut row = 0;
    let mut col = 0;
    let vec_size = full_vec.len();
    while row < vec_size {
        col = 0;
        while col < vec_size {
            if is_number(&full_vec[row][col]) {
                let num_is_valid = check_neighbours_for_symbols(
                    &full_vec,
                    row.clone(),
                    col.clone(),
                    MovedInDirection::diagonal,
                    true,
                );
                let num = get_valid_number_forward(&full_vec, row.clone(), &mut col);
                if num_is_valid {
                    println!("Num at {},{} is valid {}", row, col, num);
                    result += num;
                }
            }
            col += 1;
        }
        row += 1;
    }
    println!("{}", result);
}

fn is_asterisk(element: &char) -> bool {
    return *element == '*';
}

fn find_number_range(input: &Vec<char>, col: usize) -> (usize, usize) {
    let size = input.len();
    let mut start = col as i32;
    let mut end = col;
    while start >= 0 && is_number(&input[start as usize]) {
        start -= 1;
    }
    start += 1;

    while end < size && is_number(&input[end]) {
        end += 1;
    }

    return (start as usize, end);
}

fn check_neighours_for_numbers(input: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let size = input.len();
    let row: i32 = row as i32;
    let col: i32 = col as i32;
    let mut n_ranges: HashMap<String, (usize, usize, usize)> = HashMap::new();

    for (i, mov) in DIRECTIONS.iter().enumerate() {
        let v = validate_index(row + mov[0], &size);
        let h = validate_index(col + mov[1], &size);
        if (v.is_none() || h.is_none()) {
            continue;
        }
        let v = v.unwrap() as usize;
        let h = h.unwrap() as usize;

        if is_number(&input[v][h]) {
            let current_range = find_number_range(&input[v], h.clone());
            let full_range: (usize, usize, usize) = (v, current_range.0, current_range.1);
            let key: String = v.to_string()
                + current_range.0.to_string().as_str()
                + current_range.1.to_string().as_str();
            n_ranges.insert(key, full_range);
        }
    }

    if n_ranges.len() == 2 {
        let mut nums: [u32; 2] = [0, 0];
        for (i, element) in n_ranges.iter().enumerate() {
            nums[i] = get_valid_number_forward(&input, element.1 .0, &mut element.1 .1.clone());
        }
        println!("{:?}", n_ranges);
        println!("^ {} --- {}", nums[0], nums[1]);
        return nums[0] * nums[1];
    }

    return 0;
}

pub fn part2() {
    let mut full_vec = read_input();

    let mut result = 0;
    let mut row = 0;
    let mut col = 0;
    let vec_size = full_vec.len();
    while row < vec_size {
        col = 0;
        while col < vec_size {
            if is_asterisk(&full_vec[row][col]) {
                result += check_neighours_for_numbers(&full_vec, row.clone(), col.clone());
            }
            col += 1;
        }
        row += 1;
    }
    println!("result {}", result);
}
