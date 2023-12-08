// mod day_01;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter,
    time::{Duration, Instant},
};

pub fn task1() -> u32 {
    let lines = read();
    lines
        .iter()
        .map(|(left, right)| {
            let d1: HashSet<usize> = HashSet::from_iter(left.to_owned());
            let d2: HashSet<usize> = HashSet::from_iter(right.to_owned());
            let n = d2.intersection(&d1).count();
            if n == 0 {
                return 0;
            }
            2u32.pow((n - 1) as u32)
        })
        .sum()
}

pub fn task2() -> u32 {
    let lines = read();
    let matchs = lines.iter().map(|(left, right)| {
        let d1: HashSet<usize> = HashSet::from_iter(left.to_owned());
        let d2: HashSet<usize> = HashSet::from_iter(right.to_owned());
        d2.intersection(&d1).count()
    });

    let len = matchs.len();
    let mut cards = vec![1; len];
    for (i, value) in matchs.enumerate() {
        for j in i + 1..=i + value {
            cards[j] += cards[i];
        }
    }

    cards.iter().sum()
}

fn read() -> Vec<(Vec<usize>, Vec<usize>)> {
    let data = include_str!("data/day-04.txt");
    data.split('\n')
        .map(|line| {
            let right_part = line.split(':').collect::<Vec<_>>()[1];
            let parts = right_part.split('|').collect::<Vec<_>>();
            let (left, right) = (parts[0], parts[1]);
            let left: Vec<usize> = left
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let right: Vec<usize> = right
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (left, right)
        })
        .collect::<Vec<_>>()
}

fn sample() -> &'static str {
    include_str!("data/sample.txt")
}
