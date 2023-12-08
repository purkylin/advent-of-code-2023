// mod day_01;

pub fn task1() -> usize {
    let data = read();
    data.into_iter()
        .map(|(t, d)| calculate(t, d))
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn calculate(time: usize, distance: usize) -> usize {
    // (time - x)x > d
    // x^2 - t*x + d < 0
    // a = 1  b = -t  c = d
    // delta = t^2 - 4 * d

    let t = time as f64;
    let d = distance as f64;
    let delta = (t.powi(2) - 4. * d).sqrt();
    let tmp = delta / 2.0;

    let x1 = t / 2.0 - tmp + 1e-6;
    let x2 = t / 2.0 + tmp - 1e-6;

    let min_speed = (x1.ceil() as usize).max(1);
    let max_speed = (x2.floor() as usize).min(time);

    // println!("{} - {}", min_speed, max_speed);
    max_speed - min_speed + 1
}

pub fn task2() -> usize {
    let (time, distance) = read2();
    calculate(time, distance)
}

fn read() -> (Vec<(usize, usize)>) {
    // let data = include_str!("data/sample.txt");
    let data = include_str!("data/day-06.txt");

    let lines = data
        .split("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(x, y)| (x.to_owned(), y.to_owned()))
        .collect::<Vec<_>>()
}

fn read2() -> (usize, usize) {
    // let data = include_str!("data/sample.txt");
    let data = include_str!("data/day-06.txt");

    let lines = data
        .split("\n")
        .map(|line| line.split_ascii_whitespace().skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let t = lines[0].concat().parse::<usize>().unwrap();
    let d = lines[1].concat().parse::<usize>().unwrap();

    (t, d)
}

#[test]
fn read_data() {
    assert_eq!(calculate(7, 9), 4);
    assert_eq!(calculate(15, 40), 8);
    assert_eq!(calculate(30, 200), 9);
    assert_eq!(calculate(71530, 940200), 71503);
    assert_eq!(calculate(53897698, 313109012141201), 40651271);
}
