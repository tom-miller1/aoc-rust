use std::iter::zip;
advent_of_code::solution!(6);

fn parse_line(input: &str) -> Vec<u32> {
    input.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u32>>()
}

fn calc_wins(time: u32, distance: u32) -> u32 {
    (0..time/2+u32::from(time%2!=0))
        .map(|t| (time - t) * t)
        .filter(|x| *x > distance)
        .count() as u32 * 2 + u32::from(time%2==0)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    let pairs = zip(times, distances).collect::<Vec<(u32, u32)>>();

    Some(pairs.iter().map(|(t, d)| calc_wins(*t, *d)).product())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
