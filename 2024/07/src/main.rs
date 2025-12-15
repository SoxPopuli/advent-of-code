use common::timed;

fn can_make(target: i64, numbers: &[i64]) -> bool {
    fn do_loop(target: i64, numbers: &[i64], acc: i64) -> bool {
        match numbers {
            [] => acc == target,

            [head, tail @ ..] => {
                do_loop(target, tail, acc + head) || do_loop(target, tail, acc * head)
            }
        }
    }

    do_loop(target, &numbers[1..], numbers[0])
}

fn concat(a: i64, b: i64) -> i64 {
    let b_places = (b as f64).log10().floor() + 1.0;
    let a = a * (10_i64.pow(b_places as u32));
    a + b
}

fn can_make_concat(target: i64, numbers: &[i64]) -> bool {
    fn do_loop(target: i64, numbers: &[i64], acc: i64) -> bool {
        match numbers {
            [] => acc == target,

            [head, tail @ ..] => {
                do_loop(target, tail, acc + head)
                    || do_loop(target, tail, acc * head)
                    || do_loop(target, tail, concat(acc, *head))
            }
        }
    }

    do_loop(target, &numbers[1..], numbers[0])
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(a, b)| {
            let target = a.parse().unwrap();
            let numbers = b.split_whitespace().filter_map(|x| x.parse().ok());

            (target, numbers.collect())
        })
        .collect()
}

fn get_calibration_result(
    input: Vec<(i64, Vec<i64>)>,
    filter_func: impl Fn(i64, &[i64]) -> bool,
) -> i64 {
    input
        .iter()
        .filter(|(target, numbers)| filter_func(*target, numbers))
        .map(|(target, _)| *target)
        .sum()
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, calibration_result) = timed(|| get_calibration_result(input.clone(), can_make));
    println!("Part 1: {calibration_result} in {}μs", time.as_micros());

    let (time, calibration_result) =
        timed(|| get_calibration_result(input.clone(), can_make_concat));
    println!("Part 2: {calibration_result} in {}ms", time.as_millis());
}

// Part 1: 1399219271639 in 913μs
// Part 2: 275791737999003 in 114ms
