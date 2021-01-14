use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: String) {
    println!("Day 7, Part 1:");
    let bag_collection = parse_input(input);
    let target_bag = "shinygold";
    // let count = find_parents_recursive(target_bag.to_string(), &bag_collection);
    let count = find_parents_recursive2(target_bag.to_string(), &bag_collection)
        .iter()
        .unique()
        .count();
    println!("Counted {} bags that can contain target", count);
}

pub fn part2(input: String) {
    println!("Day 7, Part 2:");
    let (bag_collection_parents, bag_collection_children) = parse_input_improved(input);
    let start_bag = "shinygold";
    let count = count_child_bags(start_bag.to_string(), &bag_collection_parents);
    println!("Shiny Gold bag contains {} other bags", count);
}

fn find_parents_recursive(child: String, map: &HashMap<String, Vec<BagRelation>>) -> i32 {
    if map.contains_key(&child) {
        return (map[&child].len() as i32)
            + map[&child]
                .iter()
                .map(|br| find_parents_recursive(br.parent.clone(), &map))
                .sum::<i32>();
    }
    return 0;
}

fn find_parents_recursive2(child: String, map: &HashMap<String, Vec<BagRelation>>) -> Vec<String> {
    if map.contains_key(&child) {
        let mut ret_vec = map[&child].iter().map(|br| br.parent.clone()).collect_vec();
        let mut rec_vec = map[&child]
            .iter()
            .flat_map(|br| find_parents_recursive2(br.parent.clone(), &map))
            .collect_vec();

        ret_vec.append(&mut rec_vec);
        return ret_vec;
    }
    return Vec::new();
}

fn count_child_bags(parent: String, map: &HashMap<String, Vec<BagRelation>>) -> i32 {
    if map.contains_key(&parent) {
        let mut count = 1;
        for br in map[&parent].iter() {
            count += br.count * count_child_bags(br.parent.clone(), &map);
        }
        return count;
    }
    return 1;
}

fn parse_input_improved(
    input: String,
) -> (
    HashMap<String, Vec<BagRelation>>,
    HashMap<String, Vec<BagRelation>>,
) {
    let mut bag_collection_parents: HashMap<String, Vec<BagRelation>> = HashMap::new();
    let mut bag_collection_children: HashMap<String, Vec<BagRelation>> = HashMap::new();
    for l in input.lines() {
        let (a, b) = l.split(" contain ").collect_tuple().unwrap();
        let parent = a.split(" ").take(2).join("");
        if b.contains("no other") {
            bag_collection_parents.entry(parent).or_insert(Vec::new());
            continue;
        }
        for bag_entry in b.split(", ") {
            let mut split_bag_entry = bag_entry.split(" ");
            let cnt = split_bag_entry.next().unwrap().parse::<i32>().unwrap();
            let child = split_bag_entry.take(2).join("");
            bag_collection_children
                .entry(child.clone())
                .or_insert(Vec::new())
                .push(BagRelation {
                    parent: parent.clone(),
                    count: cnt,
                });
            bag_collection_parents
                .entry(parent.clone())
                .or_insert(Vec::new())
                .push(BagRelation {
                    parent: child.clone(),
                    count: cnt,
                });
        }
    }
    return (bag_collection_parents, bag_collection_children);
}

fn parse_input(input: String) -> HashMap<String, Vec<BagRelation>> {
    let mut bag_collection: HashMap<String, Vec<BagRelation>> = HashMap::new();
    for l in input.lines() {
        let (a, b) = l.split(" contain ").collect_tuple().unwrap();
        if b.contains("no other") {
            continue;
        }
        let parent = a.split(" ").take(2).join("");
        for bag_entry in b.split(", ") {
            let mut split_bag_entry = bag_entry.split(" ");
            let cnt = split_bag_entry.next().unwrap().parse::<i32>().unwrap();
            let child = split_bag_entry.take(2).join("");
            bag_collection
                .entry(child)
                .or_insert(Vec::new())
                .push(BagRelation {
                    parent: parent.clone(),
                    count: cnt,
                });
        }
    }
    return bag_collection;
}

#[derive(Debug, Default, Clone, PartialEq)]
struct BagRelation {
    parent: String,
    count: i32,
}
