use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Device {
    id: String,
    outputs: Vec<String>,
}

type Devices = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Devices {
    input
        .lines()
        .filter_map(|line| {
            let (device, outputs) = line.split_once(": ")?;

            let outputs = outputs.trim().split_ascii_whitespace();

            Some((
                device.trim().to_string(),
                outputs.map(|x| x.to_string()).collect(),
            ))
        })
        .collect()
}

fn find_paths(devices: &Devices) -> u64 {
    type Memo = HashMap<String, u64>;

    let mut seen = Memo::new();

    fn find_next(devices: &Devices, current_device: &str, seen: &mut Memo) -> u64 {
        if let Some(&x) = seen.get(current_device) {
            return x;
        }

        let mut total = 0;

        for device in devices
            .get(current_device)
            .iter()
            .flat_map(|x| x.as_slice())
        {
            if device == "out" {
                total += 1;
            } else {
                total += find_next(devices, device.as_str(), seen);
            }
        }

        seen.insert(current_device.to_string(), total);
        total
    }

    find_next(devices, "you", &mut seen)
}

fn find_paths_2(devices: &Devices) -> u64 {
    type Memo = HashMap<(String, bool, bool), u64>;

    let mut seen = Memo::new();

    fn find_next(
        devices: &Devices,
        current_device: &str,
        dac: bool,
        fft: bool,
        seen: &mut Memo,
    ) -> u64 {
        if let Some(&x) = seen.get(&(current_device.to_string(), dac, fft)) {
            return x;
        }

        let mut total = 0;

        for device in devices
            .get(current_device)
            .iter()
            .flat_map(|x| x.as_slice())
        {
            if device == "dac" {
                total += find_next(devices, device.as_str(), true, fft, seen);
            } else if device == "fft" {
                total += find_next(devices, device.as_str(), dac, true, seen);
            } else if device == "out" {
                if dac && fft {
                    total += 1;
                }
            } else {
                total += find_next(devices, device.as_str(), dac, fft, seen);
            }
        }

        seen.insert((current_device.to_string(), dac, fft), total);
        total
    }

    find_next(devices, "svr", false, false, &mut seen)
}

fn main() {
    let devices = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| find_paths(&devices));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| find_paths_2(&devices));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
        "#;
        let input = parse_input(input);
        assert_eq!(find_paths(&input), 5);
    }

    #[test]
    fn example_2() {
        let input = r#"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
        "#;
        let input = parse_input(input);
        assert_eq!(find_paths_2(&input), 2);
    }
}
