advent_of_code::solution!(7);

use std::collections::{BTreeMap, HashMap};
use crate::Hand::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Debug)]
enum Hand {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn hand_type(hand: &str) -> Hand {
    let counts: HashMap<char, u32> = hand
        .chars()
        .fold(HashMap::new(), |mut m, c| {
            *m.entry(c).or_insert(0) += 1;
            m
        });
    match counts.len() {
        1 => FiveOfAKind,
        2 => if counts.values().any(|&x| x == 3) { FullHouse } else { FourOfAKind },
        3 => if counts.values().any(|&x| x == 3) { ThreeOfAKind } else { TwoPair },
        4 => OnePair,
        5 => HighCard,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let value_map:HashMap<char, char> = HashMap::from_iter(
        "23456789TJQKA".chars().zip("abcdefghijklm".chars()).collect::<Vec<(char, char)>>()
    );

    let mut v1 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v2 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v3 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v4 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v5 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v6 : BTreeMap<String, u32> = BTreeMap::new();
    let mut v7 : BTreeMap<String, u32> = BTreeMap::new();

    for line in input.lines() {
        let fields: (&str, u32) = line.split_once(' ')
            .map(|(s, n)| (s, n.parse().unwrap())).unwrap();
        let hand = hand_type(fields.0);
        let hand_id = fields.0.chars().map(|c| value_map[&c]).collect::<String>();
        let key = hand_id + " " + fields.0;
        match hand {
            FiveOfAKind => v7.insert(key, fields.1),
            FourOfAKind => v6.insert(key, fields.1),
            FullHouse => v5.insert(key, fields.1),
            ThreeOfAKind => v4.insert(key, fields.1),
            TwoPair => v3.insert(key, fields.1),
            OnePair => v2.insert(key, fields.1),
            HighCard => v1.insert(key, fields.1),
        };
    }

    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        v1, v2, v3, v4, v5, v6, v7);

    let mut total = 0;
    for (i, v) in v1.values()
        .chain(v2.values()
        .chain(v3.values()
        .chain(v4.values()
        .chain(v5.values()
        .chain(v6.values()
        .chain(v7.values())))))).enumerate() {
        total += (1 + i as u32) * *v as u32;
        println!("{} {} total {}", 1+i, v, total);
    }

    Some(total)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
