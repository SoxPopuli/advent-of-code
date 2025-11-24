use std::collections::HashSet;

fn part_1(input: &str) -> usize {
    let blocks = input.split("\n\n");

    fn read_block(block: &str) -> HashSet<char> {
        block.lines().fold(HashSet::new(), |mut set, line| {
            line.trim().chars().for_each(|c| {
                set.insert(c);
            });

            set
        })
    }

    blocks.map(read_block).map(|x| x.len()).sum()
}

fn part_2(input: &str) -> usize {
    let blocks = input.split("\n\n");

    fn read_block(block: &str) -> Option<HashSet<char>> {
        let lines = block.lines();

        lines
            .map(|line| line.trim().chars().collect::<HashSet<_>>())
            .reduce(|set, x| set.intersection(&x).copied().collect())
    }

    blocks.filter_map(read_block).map(|x| x.len()).sum()
}

fn main() {
    let input = common::read_stdin();

    let (time, result) = common::timed(|| part_1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part_2(&input));
    println!("Part 2: {result} in {time:?}");
}
