pub fn part1(input: String) {
    println!("Day 5, Part 1:");
    let seat_list = parse_input(input);
    // println!("{:?}", seat_list);
    println!("Max seat ID is {}", seat_list.iter().map(|s| s.seat_id()).max().unwrap());
}

pub fn part2(input: String) {
    println!("Day 5, Part 2:");
    let seat_list = parse_input(input);
    let mut sorted_seat_ids = seat_list.iter().map(|s| s.seat_id()).collect::<Vec<_>>();
    sorted_seat_ids.sort();
    let mut prev_sid = 0;
    for cur_sid in sorted_seat_ids {
        if prev_sid == 0 {
            prev_sid = cur_sid;
            continue;
        }
        match cur_sid - prev_sid {
            2 => println!("Found missing seat id: {}", cur_sid-1),
            _ => prev_sid = cur_sid,
        };
    }

}

fn parse_input(input: String) -> Vec<Seat> {
    let mut seat_list: Vec<Seat> = Vec::new();
    for l in input.lines() {
        let mut s = Seat { row: 0, column: 0 };
        for (i, c) in l.chars().enumerate() {
            let bit = match c {
                'F'|'L' => 0,
                'B'|'R' => 1,
                _ => panic!("character should be one of F,B,L,R: {:?}", c),
            };
            match i {
                i if i < 7 => {
                    s.row |= bit << (6-i);
                }
                i if i < 10 => {
                    s.column |= bit << (2-(i-7));
                }
                _ => (),
            }
        }
        seat_list.push(s);
    }
    return seat_list;
}

#[derive(Debug)]
struct Seat {
    row: i32,
    column: i32,
}

impl Seat {
    fn seat_id(&self) -> i32 {
        return self.row * 8 + self.column;
    }
}
