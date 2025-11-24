#!/usr/bin/env nu

const root = path self | path dirname

def main [year: int, day: int] {
    cd $root

    let year_str = $year | into string

    let day_str = if $day < 10 {
        $"0($day)"
    } else {
        $day | into string
    }

    if not ($year_str | path exists) {
        mkdir $year_str
    }

    cd $year_str
    cargo new $day_str --name $"aoc-($year_str)-($day_str)"

    let manifest_path = $"($day_str)/Cargo.toml"

    bash -c $"echo 'common.workspace = true' >> ($manifest_path)"
}
