pub fn part1(input: String) {
    println!("Day 1, Part 1:");
    let vec = input.lines().map(|x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
    for i in &vec {
        for j in &vec {
            if (i+j) == 2020 {
                println!("{} + {} = 2020", i, j);
                println!("{} x {} = {}", i, j, i*j);
            }
        }
    }
}

pub fn part2(input: String) {
    println!("Day 1, Part 2:");
    let vec = input.lines().map(|x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
    for i in &vec {
        for j in &vec {
            for k in &vec {
                if (i+j+k) == 2020 {
                    println!("{} + {} + {} = 2020", i, j, k);
                    println!("{} x {} x {} = {}", i, j, k, i*j*k);
                }
            }
        }
    }
}
