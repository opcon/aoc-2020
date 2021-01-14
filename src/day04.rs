use regex::Regex;
use std::fmt;

pub fn part1(input: String) {
    println!("Day 4, Part 1:");
    let pass_collection = parse_input(&input);
    println!(
        "{} valid passports",
        pass_collection.iter().filter(|x| x.is_valid()).count()
    );
}

pub fn part2(input: String) {
    println!("Day 4, Part 2:");
    let pass_collection = parse_input_strict(&input);
    println!(
        "{} valid passports",
        pass_collection.iter().filter(|x| x.is_valid()).count()
    );
}

fn parse_input(input: &String) -> Vec<Passport> {
    let mut passport_collection: Vec<Passport> = Vec::new();
    let mut current_passport: Passport = Passport::default();
    for l in input.lines() {
        if l.is_empty() {
            passport_collection.push(current_passport);
            current_passport = Passport::default();
        }

        for field in l.split(' ').map(|s| s.split(':').collect::<Vec<_>>()) {
            match field[0] {
                "byr" => current_passport.byr = field[1],
                "iyr" => current_passport.iyr = field[1],
                "eyr" => current_passport.eyr = field[1],
                "hgt" => current_passport.hgt = field[1],
                "hcl" => current_passport.hcl = field[1],
                "ecl" => current_passport.ecl = field[1],
                "pid" => current_passport.pid = field[1],
                "cid" => current_passport.cid = field[1],
                _ => (),
            };
        }
    }
    // Push final passport
    passport_collection.push(current_passport);
    return passport_collection;
}

fn parse_input_strict(input: &String) -> Vec<StrictPassport> {
    let mut passport_collection: Vec<StrictPassport> = Vec::new();
    let mut current_passport: StrictPassport = StrictPassport::default();
    let height_regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    let haircolor_regex = Regex::new(r"#[0-9,a-f]{6}$").unwrap();
    let passport_number_regex = Regex::new(r"^\d{9}$").unwrap();
    for l in input.lines() {
        if l.is_empty() {
            passport_collection.push(current_passport);
            current_passport = StrictPassport::default();
        }

        for field in l.split(' ').map(|s| s.split(':').collect::<Vec<_>>()) {
            match field[0] {
                "byr" => {
                    if field[1].len() == 4 {
                        match field[1].parse::<i32>().ok() {
                            Some(i) if i >= 1920 && i <= 2002 => current_passport.byr = Some(i),
                            _ => (),
                        }
                    }
                }
                "iyr" => {
                    if field[1].len() == 4 {
                        match field[1].parse::<i32>().ok() {
                            Some(i) if i >= 2010 && i <= 2020 => current_passport.iyr = Some(i),
                            _ => (),
                        }
                    }
                }
                "eyr" => {
                    if field[1].len() == 4 {
                        match field[1].parse::<i32>().ok() {
                            Some(i) if i >= 2020 && i <= 2030 => current_passport.eyr = Some(i),
                            _ => (),
                        }
                    }
                }
                "hgt" => match height_regex.captures(field[1]) {
                    Some(c) => {
                        match c.get(2).unwrap().as_str() {
                            "cm" => current_passport.hgt_unit = Some(HeightUnit::Cm),
                            "in" => current_passport.hgt_unit = Some(HeightUnit::In),
                            _ => (),
                        }
                        match c.get(1).unwrap().as_str().parse::<i32>().ok() {
                            Some(h)
                                if (current_passport.hgt_unit.unwrap() == HeightUnit::Cm
                                    && h >= 150
                                    && h <= 193)
                                    || (current_passport.hgt_unit.unwrap() == HeightUnit::In
                                        && h >= 59
                                        && h <= 76) =>
                            {
                                current_passport.hgt = Some(h)
                            }
                            _ => (),
                        }
                    }
                    None => (),
                },
                "hcl" => match haircolor_regex.is_match(field[1]) {
                    true => {
                        current_passport.hcl = Some(field[1]);
                    }
                    false => (),
                },
                "ecl" => match field[1] {
                    "amb" => current_passport.ecl = Some(EyeColor::Amber),
                    "blu" => current_passport.ecl = Some(EyeColor::Blue),
                    "brn" => current_passport.ecl = Some(EyeColor::Brown),
                    "gry" => current_passport.ecl = Some(EyeColor::Gray),
                    "grn" => current_passport.ecl = Some(EyeColor::Green),
                    "hzl" => current_passport.ecl = Some(EyeColor::Hazel),
                    "oth" => current_passport.ecl = Some(EyeColor::Other),
                    _ => (),
                },
                "pid" => match passport_number_regex.is_match(field[1]) {
                    true => {
                        current_passport.pid = field[1].parse::<i32>().ok();
                    }
                    false => (),
                },
                _ => (),
            };
        }
    }
    // Push final passport
    passport_collection.push(current_passport);
    return passport_collection;
}

#[derive(Default)]
struct Passport<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: &'a str,
}

impl fmt::Display for Passport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "byr:{},iyr:{},eyr:{},hgt:{},hcl:{},ecl:{},pid:{},cid:{}",
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid, self.cid
        )
    }
}

impl Passport<'_> {
    fn is_valid(&self) -> bool {
        return !(self.byr.is_empty()
            || self.iyr.is_empty()
            || self.eyr.is_empty()
            || self.hgt.is_empty()
            || self.hcl.is_empty()
            || self.ecl.is_empty()
            || self.pid.is_empty());
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum HeightUnit {
    Cm,
    In,
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

#[derive(Default, Debug)]
struct StrictPassport<'a> {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<i32>,
    hgt_unit: Option<HeightUnit>,
    hcl: Option<&'a str>,
    ecl: Option<EyeColor>,
    pid: Option<i32>,
    cid: Option<i32>,
}

impl fmt::Display for StrictPassport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StrictPassport<'_> {
    fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }
}
