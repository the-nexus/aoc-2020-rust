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
    find_numbers_mul_where_sum_equals_2020(input, 2, 0, 0, 1)
}

#[aoc(day1, part2)]
fn part_two(input: &Vec<i32>) -> i32 {
    find_numbers_mul_where_sum_equals_2020(input, 3, 0, 0, 1)
}

fn find_numbers_mul_where_sum_equals_2020(input: &Vec<i32>, count: usize, start: usize, sum: i32, mul: i32) -> i32 {
    if count == 0 {
        return if sum == 2020 { mul } else { -1 };
    }
    else if sum < 2020 {
        let end = input.len() - count;
        for idx in start..end {
            let next_sum = sum + input[idx];
            let result = find_numbers_mul_where_sum_equals_2020(input, count - 1, idx + 1, next_sum, mul * input[idx]);
            if result != -1 { return result; }
        }
    }
    -1
}



///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]

#[test]
fn test_part_one() {
    let sample = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(514579, part_one(&sample));
}

#[test]
fn test_part_two() {
    let sample = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(241861950, part_two(&sample));
}