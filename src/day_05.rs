// mod day_01;
use std::{
    clone,
    collections::{HashMap, HashSet},
    fmt::Display,
    iter,
    ops::Range,
    time::{Duration, Instant},
};

pub fn task1() -> usize {
    let (seeds, maps) = read();
    let seeds = seeds.into_iter().map(|x| x..x + 1).collect();

    let mut input = seeds;
    for i in 0..maps.len() {
        input = calculate_layer(input, &maps[i]);
    }

    input.into_iter().map(|x| x.start).min().unwrap()
}

pub fn task2() -> usize {
    let (seeds, maps) = read();
    // let seeds = seeds.into_iter().map(|x| x..x + 1).collect();
    let seeds = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();

    let mut input = seeds;
    for i in 0..maps.len() {
        input = calculate_layer(input, &maps[i]);
    }

    input.into_iter().map(|x| x.start).min().unwrap()
}

fn calculate_layer(
    input: Vec<Range<usize>>,
    layer: &Vec<(usize, usize, usize)>,
) -> Vec<Range<usize>> {
    let mut todos = input;

    let mut output = vec![];

    for rule in layer {
        let rule_range = rule.1..rule.1 + rule.2;
        let items = todos
            .iter()
            .map(|item| split_range(item, &rule_range))
            .flatten()
            .collect::<Vec<_>>();
        todos.clear();

        for item in items {
            if is_overlap(&item, &rule_range) {
                let offset = (rule.0 as i64) - (rule.1 as i64);
                let new_item = range_with_offset(item, offset);
                output.push(new_item);
            } else {
                todos.push(item)
            }
        }
    }

    output.append(&mut todos);
    output
}

fn range_with_offset(range: Range<usize>, offset: i64) -> Range<usize> {
    let start = ((range.start as i64) + offset) as usize;
    let end = ((range.end as i64) + offset) as usize;
    start..end
}

fn is_overlap(lhs: &Range<usize>, to: &Range<usize>) -> bool {
    to.start <= lhs.start && to.end >= lhs.end
}

fn split_range(from: &Range<usize>, to: &Range<usize>) -> Vec<Range<usize>> {
    if from.end < to.start || from.start > to.end {
        return vec![from.start..from.end];
    }

    let mut arr = vec![];
    if from.start < to.start {
        arr.push(from.start..to.start.min(from.end));
        if from.end < to.end {
            arr.push(to.start..from.end)
        }
    } else {
        arr.push(from.start..to.end.min(from.end));
        if from.end > to.end {
            arr.push(to.end..from.end);
        }
    }

    arr.into_iter().filter(|x| !x.is_empty()).collect()
}

// #[test]
// fn test_range() {
//     // overlap
//     let r1 = 2..10;
//     let r2 = 3..12;
//     assert_eq!(split_range(r1, r2), vec![2..3, 3..10]);

//     let r1 = 5..8;
//     let r2 = 1..6;
//     assert_eq!(split_range(r1, r2), vec![5..6, 6..8]);

//     // eq
//     let r1 = 1..6;
//     let r2 = 1..6;
//     assert_eq!(split_range(r1, r2), vec![1..6]);

//     // not overlap
//     let r1 = 1..6;
//     let r2 = 8..12;
//     assert_eq!(split_range(r1, r2), vec![1..6]);

//     // upper boundary
//     let r1 = 1..6;
//     let r2 = 6..12;
//     assert_eq!(split_range(r1, r2), vec![1..6]);

//     // lower boundary
//     let r1 = 12..13;
//     let r2 = 6..12;
//     assert_eq!(split_range(r1, r2), vec![12..13]);

//     // contain
//     let r1 = 12..13;
//     let r2 = 6..14;
//     assert_eq!(split_range(r1, r2), vec![12..13]);
// }

fn find_destination(number: usize, maps: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut value = number;

    for detail in maps {
        for (target, source, range) in detail {
            let source = source.to_owned();
            let range = range.to_owned();
            if (source..source + range).contains(&value) {
                value = value - source + target;
                break;
            }
        }
    }

    value
}

fn read() -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    // let data = include_str!("data/sample.txt");
    let data = include_str!("data/day-05.txt");

    let mut sections = data.split("\n\n");
    let seed_section = sections.nth(0).expect("Invalid data");
    let seeds: Vec<usize> = seed_section
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let maps = sections
        // .skip(1)
        .map(|section| {
            let lines = section.split("\n");
            lines
                .skip(1)
                .map(|line| {
                    let numbers = line
                        .split_ascii_whitespace()
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    assert!(numbers.len() == 3);
                    (numbers[0], numbers[1], numbers[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn sample() -> &'static str {
    include_str!("data/sample.txt")
}
