use std::fmt::Display;

pub trait AdventDay {
    type T;

    fn part1(&self) -> Self::T;
    fn part2(&self) -> Self::T;
}

pub struct Day {}

impl AdventDay for Day {
    type T = u8;

    fn part1(&self) -> Self::T {
        12
    }

    fn part2(&self) -> Self::T {
        21
    }
}

enum Answer {
    String(String),
    Number(u64),
    Float(f64),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::String(v) => write!(f, "{v}"),
            Answer::Number(v) => write!(f, "{v}"),
            Answer::Float(v) => write!(f, "{v}"),
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u8>
