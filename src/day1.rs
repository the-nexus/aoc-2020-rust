use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| {
            line.parse::<i32>().expect("")
        })
        .collect::<Vec<i32>>()
}

#[aoc(day1, part1)]
fn part_one(input: &Vec<i32>) -> i32 {
    input[0]
}

#[aoc(day1, part2)]
fn part_two(input: &Vec<i32>) -> i32 {
    input[0]
}
