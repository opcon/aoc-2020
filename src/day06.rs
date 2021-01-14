use std::collections::HashMap;

pub fn part1(input: String) {
    println!("Day 6, Part 1:");
    println!("Sum is {}", do_p1(input));
}

pub fn part2(input: String) {
    println!("Day 6, Part 2:");
    println!("Sum is {}", do_p2(input));
}

fn do_p1(input: String) -> i32 {
    let mut questions: HashMap<char, i32> = HashMap::new();
    let mut sum = 0i32;

    for l in input.lines() {
        if l.is_empty() {
            sum += questions.iter().filter(|(&_,&v)| v == 1).count() as i32;
            questions.clear();
            continue;
        }
        for c in l.chars() {
            questions.entry(c).or_insert(1);
        }
    }
    sum += questions.iter().filter(|(&_,&v)| v == 1).count() as i32;
    return sum;
}

fn do_p2(input: String) -> i32 {
    let mut questions: HashMap<char, i32> = HashMap::new();
    let mut sum = 0i32;
    let mut group_members = 0;

    for l in input.lines() {
        if l.is_empty() {
            sum += questions.iter().filter(|(&_,&v)| v == group_members).count() as i32;
            questions.clear();
            group_members = 0;
            continue;
        }
        for c in l.chars() {
            *questions.entry(c).or_insert(0) += 1;
        }
        group_members += 1;
    }
    sum += questions.iter().filter(|(&_,&v)| v == group_members).count() as i32;
    return sum;
}
