
pub fn part1(input: String) {
    println!("Day 3, Part 1:");
    let map = parse_input_map(&input);
    let width = map[0].len() as usize;
    let height = map.len() as usize;
    let mut position = Point { x: 0, y: 0}; 
    println!("Map WxH = {}x{}", width, height);
    
    let mut tree_count = 0;
    loop {
        match map[position.y % height][position.x % width] {
            '#' => tree_count += 1,
            _ => (),
        }
        position.x += 3;
        position.y += 1;
        if position.y >= height {
            break;
        }
    }
    println!("Counted {} trees", tree_count);
}

pub fn part2(input: String) {
    println!("Day 3, Part 2:");
    let map = parse_input_map(&input);
    let directions: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut tree_mult = 1;
    for d in directions {
        tree_mult *= check_direction(&map, d);
    }

    println!("Multiple of trees is {}", tree_mult);
}

fn check_direction(map: &Vec<Vec<char>>, dir: (usize, usize)) -> i32 {
    let mut tree_count = 0;
    let mut position = Point { x: 0, y: 0}; 
    let width = map[0].len() as usize;
    let height = map.len() as usize;
    loop {
        match map[position.y % height][position.x % width] {
            '#' => tree_count += 1,
            _ => (),
        }
        position.x += dir.0;
        position.y += dir.1;
        if position.y >= height {
            break;
        }
    }
    return tree_count
}

fn parse_input_map<'a>(input: &'a String) -> Vec<Vec<char>> {
    return input.lines().map(|x| x.chars().collect()).collect()
}

struct Point {
    x: usize,
    y: usize,
}
