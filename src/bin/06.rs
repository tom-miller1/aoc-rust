use std::iter::zip;
advent_of_code::solution!(6);

fn parse_line(input: &str) -> Vec<u64> {
    input.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u64>>()
}

fn parse_line_part2(input: &str) -> u64 {
    input.split_whitespace().skip(1).collect::<String>().parse().unwrap()
}

fn calc_wins(time: u64, distance: u64) -> u64 {
    (0..time/2+u64::from(time%2!=0))
        .map(|t| (time - t) * t)
        .filter(|x| *x > distance)
        .count() as u64 * 2 + u64::from(time%2==0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    let pairs = zip(times, distances).collect::<Vec<(u64, u64)>>();

    Some(pairs.iter().map(|(t, d)| calc_wins(*t, *d)).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = parse_line_part2(lines.next().unwrap());
    let distance = parse_line_part2(lines.next().unwrap());

    Some(calc_wins(time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
