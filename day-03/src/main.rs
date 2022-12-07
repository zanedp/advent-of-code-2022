use std::collections::HashSet;

fn priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => (ch as u32) - ('a' as u32) + 1,
        'A'..='Z' => (ch as u32) - ('A' as u32) + 27,
        _ => panic!("That's not a valid char"),
    }
}
fn main() {
    let _contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let contents = include_str!("input.txt");
    let mut priority_sum = 0;
    for line in contents.lines() {
        let (front, back) = line.split_at(line.len() / 2);
        let set: HashSet<char> = HashSet::from_iter(front.chars());
        let common = back.chars().find(|ch| set.contains(ch)).unwrap();
        priority_sum += priority(common);
    }
    println!("both sum = {priority_sum}");

    let mut badge_sum = 0;
    for lines in contents.lines().collect::<Vec<_>>().chunks(3) {
        let a = lines[0];
        let b = lines[1];
        let c = lines[2];
        let set_a: HashSet<char> = HashSet::from_iter(a.chars());
        let set_b: HashSet<char> = HashSet::from_iter(b.chars());
        let badge = c
            .chars()
            .find(|ch| set_a.contains(ch) && set_b.contains(ch))
            .unwrap();
        badge_sum += priority(badge);
    }
    println!("badge sum = {badge_sum}");
}

#[test]
fn test_priority() {
    assert_eq!(1, priority('a'));
    assert_eq!(26, priority('z'));
    assert_eq!(27, priority('A'));
    assert_eq!(52, priority('Z'));
}
