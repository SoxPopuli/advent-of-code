// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bag {
    bag: String,
    children: Vec<(u32, String)>,
}

fn read_line(line: &str) -> Bag {
    let (bag, rest) = line
        .split_once(" bags contain ")
        .expect("invalid line input");

    let mut words = rest.split_whitespace().peekable();

    let children = {
        if let Some(next) = words.peek()
            && *next == "no"
        {
            vec![]
        } else {
            let mut bags = vec![];

            while let Some(next) = words.next() {
                if let Ok(count) = next.parse::<u32>() {
                    let color_1 = words.next();
                    let color_2 = words.next();

                    if let (Some(a), Some(b)) = (color_1, color_2) {
                        let bag = format!("{a} {b}");
                        bags.push((count, bag));
                    }
                }
            }

            bags
        }
    };

    Bag {
        bag: bag.into(),
        children,
    }
}

type BagMap = HashMap<String, Vec<(u32, String)>>;
type ReverseBagMap = HashMap<String, Vec<String>>;

fn build_reverse_bag_map(bags: impl IntoIterator<Item = Bag>) -> ReverseBagMap {
    let mut map = HashMap::new();

    for b in bags {
        for (_, child) in &b.children {
            map.entry(child.clone())
                .and_modify(|contained: &mut Vec<String>| contained.push(b.bag.clone()))
                .or_insert_with(|| vec![b.bag.clone()]);
        }
    }

    map
}

fn contained_in_count(bag_map: &ReverseBagMap, visited: &mut HashSet<String>, bag: &str) -> usize {
    let mut sum = 0;

    if let Some(contains) = bag_map.get(bag) {
        for c in contains.iter() {
            if !visited.contains(c) {
                sum += 1;
                visited.insert(c.clone());

                sum += contained_in_count(bag_map, visited, c);
            }
        }
    }

    sum
}

fn part_1(bags: impl IntoIterator<Item = Bag>) -> usize {
    let bag_map = build_reverse_bag_map(bags);
    contained_in_count(&bag_map, &mut HashSet::new(), "shiny gold")
}

fn part_2(bags: &[Bag]) -> usize {
    let bag_map = bags
        .iter()
        .map(|bag| (bag.bag.clone(), bag.children.clone()))
        .collect::<HashMap<_, _>>();

    fn contains_count(bag_map: &BagMap, bag: &str) -> usize {
        let children = bag_map.get(bag);
        match children {
            None => 0,
            Some(children) => {
                if children.is_empty() {
                    1
                } else {
                    let mut sum = 1;
                    for (count, child) in children {
                        sum += *count as usize * contains_count(bag_map, child);
                    }

                    sum
                }
            }
        }
    }

    let bag = bags.iter().find(|bag| bag.bag == "shiny gold").unwrap();

    contains_count(&bag_map, &bag.bag) - 1
}

fn main() {
    let input = common::read_stdin();

    let bags = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(read_line)
        .collect::<Vec<_>>();

    let (time, result) = common::timed(|| part_1(bags.iter().cloned()));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part_2(&bags));
    println!("Part 2: {result} in {time:?}");
}
