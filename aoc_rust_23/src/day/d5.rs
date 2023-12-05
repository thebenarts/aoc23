use std::ops::RangeBounds;

use crate::aoc_reader;

#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<Range>,
}

fn read_input() -> (Vec<u64>, Vec<Mapping>) {
    let input = aoc_reader::read_day_input(5);
    let mut blocks = input.split("\r\n\r\n");

    let seeds: Vec<u64> = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    println!("{:?}", seeds);
    let mut mappings: Vec<Mapping> = Vec::new();
    for block in blocks {
        let mut lines = block.lines();
        lines.next(); // eat the first useless line
        let mut mapping: Mapping = Mapping { ranges: Vec::new() };

        for line in lines {
            //println!("{line}");
            let mut split = line.split_whitespace();
            mapping.ranges.push(Range {
                destination: split.next().unwrap().parse().unwrap(),
                source: split.next().unwrap().parse().unwrap(),
                length: split.next().unwrap().parse().unwrap(),
            });
        }

        mappings.push(mapping);
    }

    return (seeds, mappings);
}

fn find_location(seed: u64, mappings: &Vec<Mapping>) -> u64 {
    let mut current_seed = seed;
    for mapping in mappings {
        for range in mapping.ranges.iter() {
            if current_seed >= range.source && current_seed < range.source + range.length {
                current_seed = range.destination + current_seed - range.source;
                break;
            }
        }
    }
    return current_seed;
}

use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn part1() {
    let (seeds, mappings) = read_input();
    let mut min = std::u64::MAX;
    for seed in seeds {
        min = std::cmp::min(min, find_location(seed.clone(), &mappings));
    }

    println!("{}", min);
}

pub fn part2() {
    let (seeds, mappings) = read_input();
    let arc_mappings: Arc<Vec<Mapping>> = Arc::new(mappings);

    let mut thread_vec: Vec<thread::JoinHandle<(u64)>> = Vec::new();
    for i in (0..20).step_by(2) {
        let seed1 = seeds[i];
        let seed2 = seeds[i + 1];

        let arc_map = Arc::clone(&arc_mappings);
        thread_vec.push(thread::spawn(move || {
            let mut min = std::u64::MAX;
            for n in 0..seed2 {
                min = std::cmp::min(min, find_location(seed1 + n, &arc_map));
            }
            return min;
        }));
    }

    let mut min = std::u64::MAX;
    for handle in thread_vec {
        min = std::cmp::min(min, handle.join().unwrap());
    }
    println!("{min}");
}
