use std::collections::HashSet;

use common::timed;

thread_local! {
    static REQUIRED_FIELDS: HashSet<Field> = HashSet::from_iter([
        Field::BirthYear,
        Field::IssueYear,
        Field::ExpirationYear,
        Field::Height,
        Field::HairColor,
        Field::EyeColor,
        Field::PassportID,
    ]);
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}
impl Field {
    fn from_key(key: &str) -> Self {
        match key {
            "byr" => Self::BirthYear,
            "iyr" => Self::IssueYear,
            "eyr" => Self::ExpirationYear,
            "hgt" => Self::Height,
            "hcl" => Self::HairColor,
            "ecl" => Self::EyeColor,
            "pid" => Self::PassportID,
            "cid" => Self::CountryID,

            _ => panic!("Unexpected key: {key}"),
        }
    }

    fn validate(&self, value: &str) -> bool {
        fn validate_year(value: &str, min: usize, max: usize) -> bool {
            if value.len() == 4 {
                value
                    .parse::<usize>()
                    .ok()
                    .filter(|x| *x >= min && *x <= max)
                    .map(|_| true)
                    .unwrap_or(false)
            } else {
                false
            }
        }

        match self {
            Field::BirthYear => validate_year(value, 1920, 2002),
            Field::IssueYear => validate_year(value, 2010, 2020),
            Field::ExpirationYear => validate_year(value, 2020, 2030),
            Field::Height => {
                if let Some(prefix) = value.strip_suffix("cm") {
                    matches!(prefix.parse::<usize>(), Ok(num) if (150..=193).contains(&num))
                } else if let Some(prefix) = value.strip_suffix("in") {
                    matches!(prefix.parse::<usize>(), Ok(num) if (59..=76).contains(&num))
                } else {
                    false
                }
            }
            Field::HairColor => {
                if let Some(suffix) = value.strip_prefix("#") {
                    suffix.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
                } else {
                    false
                }
            }
            Field::EyeColor => {
                matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
            }
            Field::PassportID => value.len() == 9 && value.parse::<usize>().is_ok(),
            Field::CountryID => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FieldEntry {
    field: Field,
    value: String,
}

type Input = Vec<Vec<FieldEntry>>;

fn parse_input(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|x| {
            let x = x.trim();

            x.split_whitespace()
                .map(|pair| {
                    let (key, value) = pair.split_once(":").unwrap();

                    FieldEntry {
                        field: Field::from_key(key),
                        value: value.to_string(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|x| {
            x.iter()
                .map(|field| field.field.clone())
                .collect::<HashSet<_>>()
        })
        .filter(|fields| REQUIRED_FIELDS.with(|req| req.is_subset(fields)))
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|x| {
            let field_set = x
                .iter()
                .map(|field| field.field.clone())
                .collect::<HashSet<_>>();

            let has_required = REQUIRED_FIELDS.with(|req| req.is_subset(&field_set));

            if has_required {
                x.iter().all(|entry| entry.field.validate(&entry.value))
            } else {
                false
            }
        })
        .count()
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, result) = timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = timed(|| part2(&input));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod test {}
