use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn task1() -> usize {
    let (instructions, rules) = read(false);
    let map = rules.iter().fold(HashMap::new(), |mut acc, (x, y, z)| {
        acc.insert(x.as_str(), (y.as_str(), z.as_str()));
        acc
    });

    let start = "AAA";
    let end = "ZZZ";

    let mut current = start;

    let mut cnt = 0;
    'a: loop {
        for c in instructions.chars() {
            cnt += 1;

            // println!("before [{}] current: {}", c, current);
            match c {
                'L' => current = map.get(current).unwrap().0,
                'R' => current = map.get(current).unwrap().1,
                _ => panic!("Invalid instruction"),
            }

            if current == end {
                break 'a;
            }
        }
    }

    cnt
}

pub fn task2() -> usize {
    let (instructions, rules) = read(false);
    let map = rules.iter().fold(HashMap::new(), |mut acc, (x, y, z)| {
        acc.insert(x.as_str(), (y.as_str(), z.as_str()));
        acc
    });

    let starts = map
        .keys()
        .filter(|key| key.ends_with("A"))
        // .take(1)
        .collect::<Vec<_>>();

    println!("starts: {:?}", starts);

    let ttt = starts
        .into_iter()
        .map(|x| calculate_count(&instructions, x, &map))
        .collect::<Vec<_>>();

    ttt.into_iter().reduce(|acc, x| lcm(acc, x)).unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn calculate_count(instructions: &str, start: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let starts = vec![start];
    let mut currents = starts;

    let mut cnt = 0;

    'a: loop {
        for c in instructions.chars() {
            cnt += 1;

            match c {
                'L' => currents.iter_mut().for_each(|x| {
                    let k = *x;
                    *x = &map.get(k).unwrap().0;
                }),
                'R' => currents.iter_mut().for_each(|x| {
                    let k = *x;
                    *x = &map.get(k).unwrap().1;
                }),
                _ => panic!("Invalid instruction"),
            }

            if currents.iter().all(|x| x.ends_with('Z')) {
                break 'a;
            }
        }
    }

    cnt
}

fn read(sample: bool) -> (String, Vec<(String, String, String)>) {
    let data: &str;
    if sample {
        data = include_str!("data/sample.txt");
    } else {
        data = include_str!("data/day-08.txt");
    }

    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let (left, right) = data.split_once("\n\n").unwrap();
    let instructions = left.to_owned();
    let rules = right
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let n1 = caps.get(1).unwrap().as_str().to_string();
            let n2 = caps.get(2).unwrap().as_str().to_string();
            let n3 = caps.get(3).unwrap().as_str().to_string();
            (n1, n2, n3)
        })
        .collect();
    (instructions, rules)
}

// #[test]
fn test_input() {
    let (instructions, rules) = read(true);
    assert_eq!(instructions, "LLR");
    let rule = &rules[0];
    assert_eq!(rule.0, "AAA");
    assert_eq!(rule.1, "BBB");
    assert_eq!(rule.2, "BBB");
}

// #[test]
fn test_sample() {
    let (instructions, rules) = read(true);
    let map = rules.iter().fold(HashMap::new(), |mut acc, (x, y, z)| {
        acc.insert(x.as_str(), (y.as_str(), z.as_str()));
        acc
    });

    let start = "AAA";
    let end = "ZZZ";

    let mut current = start;

    let mut cnt = 0;
    'a: loop {
        for c in instructions.chars() {
            cnt += 1;

            match c {
                'L' => current = map.get(current).unwrap().0,
                'R' => current = map.get(current).unwrap().1,
                _ => panic!("Invalid instruction"),
            }
            println!("after [{}] current: {}", c, current);

            if current == end {
                break 'a;
            }
        }
    }

    assert_eq!(cnt, 6);
}

#[test]
fn test_sample_part2() {
    let (instructions, rules) = read(true);
    let map = rules.iter().fold(HashMap::new(), |mut acc, (x, y, z)| {
        acc.insert(x.as_str(), (y.as_str(), z.as_str()));
        acc
    });

    let starts = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    let mut currents = starts;

    let mut cnt = 0;
    'a: loop {
        for c in instructions.chars() {
            cnt += 1;

            match c {
                'L' => currents.iter_mut().for_each(|x| {
                    let k = *x;
                    *x = &map.get(k).unwrap().0;
                }),
                'R' => currents.iter_mut().for_each(|x| {
                    let k = *x;
                    *x = &map.get(k).unwrap().1;
                }),
                _ => panic!("Invalid instruction"),
            }

            if currents.iter().all(|x| x.ends_with('Z')) {
                break 'a;
            }
        }
    }

    assert_eq!(cnt, 6);
}

#[test]
fn test_gcd() {
    assert_eq!(lcm(6usize, 8usize), 24);
}
