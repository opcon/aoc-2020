use regex::Regex;
use std::collections::HashMap;
use itertools::izip;

pub fn part1(input: String) {
    println!("Day 2, Part 1:");
    let re = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)\s+(?P<letter>[a-z,A-Z]):\s+(?P<pass>.+)").unwrap();
    let mut lower_limits = Vec::new();
    let mut upper_limits = Vec::new();
    let mut letter = Vec::new();
    let mut letter_frequency_map: Vec<HashMap<char, i32>> = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        lower_limits.push(caps.name("lower").unwrap().as_str().parse::<i32>());
        upper_limits.push(caps.name("upper").unwrap().as_str().parse::<i32>());
        letter.push(caps.name("letter").unwrap().as_str().chars().next());
        let pass = caps.name("pass").unwrap().as_str();
        let mut hm = HashMap::new();
        for c in pass.chars() {
            *hm.entry(c).or_insert(0) += 1;
        }
        letter_frequency_map.push(hm);
    }

    let mut valid_count = 0;
    for (l, u, lt, f) in izip!(lower_limits, upper_limits, letter, letter_frequency_map) {
        let cnt = match f.contains_key(&lt.unwrap()) {
            false => 0,
            true => f[&lt.unwrap()],
        };
        if (cnt >= l.unwrap()) && (cnt <= u.unwrap()) {
            // println!("Valid password");
            valid_count += 1;
        }
    }

    println!("Found {} valid passwords", valid_count);
}

pub fn part2(input: String) {
    println!("Day 2, Part 2:");
    let re = Regex::new(r"(?P<a>\d+)-(?P<b>\d+)\s+(?P<letter>[a-z,A-Z]):\s+(?P<pass>.+)").unwrap();

    let mut valid_count = 0;
    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let a = caps.name("a").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let b = caps.name("b").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let lt = caps.name("letter").unwrap().as_str().chars().next().unwrap();
        let pass = caps.name("pass").unwrap().as_str();

        let mut in_a = false;
        let mut in_b = false;

        // println!("len: {}, a: {}, b: {}, lt: {}, p: {}", pass.len(), a, b, lt, pass);

        if a < pass.len() {
            in_a = pass.chars().nth(a).unwrap() == lt;
        }
        if b < pass.len() {
            in_b = pass.chars().nth(b).unwrap() == lt;
        }

        if in_a ^ in_b {
            valid_count += 1;
        }
    }

    println!("Found {} valid passwords", valid_count);
}
