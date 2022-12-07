use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug)]
struct UniqueWindowDetector {
    q: VecDeque<char>,
    counters: HashMap<char, usize>,
    window_size: usize,
    seq: usize,
}

impl UniqueWindowDetector {
    fn new(window_size: usize) -> Self {
        Self {
            window_size,
            ..Self::default()
        }
    }

    fn add_char(&mut self, ch: char) {
        if self.seq >= self.window_size {
            // done booting up, so gotta push one out
            let evicted = self.q.pop_front().expect("queue should be full");
            self.counters.entry(evicted).and_modify(|count| *count -= 1);
            if self.counters[&evicted] == 0 {
                self.counters.remove(&evicted);
            }
        }
        self.q.push_back(ch);
        self.counters
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.seq += 1;
    }

    fn is_unique_window(&self) -> bool {
        self.counters.len() == self.window_size
    }

    fn location(&self) -> usize {
        self.seq
    }
}

fn find_start_of_unique_seq(input: &str, len: usize) -> usize {
    let mut detector = UniqueWindowDetector::new(len);
    // 4 for part 1, 14 for part 2
    for ch in input.chars() {
        detector.add_char(ch);
        if detector.is_unique_window() {
            break;
        }
    }
    detector.location()
}

fn main() {
    let sample1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let sample2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let sample3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let sample4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let sample5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let input = include_str!("input.txt");

    assert_eq!(7, find_start_of_unique_seq(sample1, 4));
    assert_eq!(5, find_start_of_unique_seq(sample2, 4));
    assert_eq!(6, find_start_of_unique_seq(sample3, 4));
    assert_eq!(10, find_start_of_unique_seq(sample4, 4));
    assert_eq!(11, find_start_of_unique_seq(sample5, 4));

    let part1_loc = find_start_of_unique_seq(input, 4);
    println!("part 1 start at char {}", part1_loc);
    assert_eq!(1896, part1_loc);

    assert_eq!(19, find_start_of_unique_seq(sample1, 14));
    assert_eq!(23, find_start_of_unique_seq(sample2, 14));
    assert_eq!(23, find_start_of_unique_seq(sample3, 14));
    assert_eq!(29, find_start_of_unique_seq(sample4, 14));
    assert_eq!(26, find_start_of_unique_seq(sample5, 14));

    let part2_loc = find_start_of_unique_seq(input, 14);
    println!("part 2 start at char {}", part2_loc);
    assert_eq!(3452, part2_loc);
}
