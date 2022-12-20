use std::collections::HashSet;

#[allow(dead_code)]
const SAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (left, right) = line.split_once(',').unwrap();

    let (left_start, left_end) = left.split_once('-').unwrap();
    let left = (left_start.parse().unwrap(), left_end.parse().unwrap());

    let (right_start, right_end) = right.split_once('-').unwrap();
    let right = (right_start.parse().unwrap(), right_end.parse().unwrap());

    (left, right)
}

fn main() {
    //let input = SAMPLE;
    let input = include_str!("input.txt");
    let mut subset_count = 0;
    let mut any_overlap_count = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        let left_set: HashSet<u32> = HashSet::from_iter(left.0..=left.1);
        let right_set: HashSet<u32> = HashSet::from_iter(right.0..=right.1);
        if left_set.is_subset(&right_set) || right_set.is_subset(&left_set) {
            subset_count += 1;
        }
        if !left_set.is_disjoint(&right_set) {
            any_overlap_count += 1;
        }
    }
    println!("# subsets = {}", subset_count);
    assert_eq!(644, subset_count, "part 1 is incorrect");
    println!("# any overlaps = {}", any_overlap_count);
    assert_eq!(926, any_overlap_count, "part 2 is incorrect");
}
