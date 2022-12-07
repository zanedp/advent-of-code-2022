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

fn main() {
    let _sample1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let _sample2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let _sample3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let _sample4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let _sample5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    let _input_file = include_str!("input.txt");

    let input = _input_file;

    let mut detector = UniqueWindowDetector::new(14); // 4 for part 1, 14 for part 2
    for ch in input.chars() {
        detector.add_char(ch);
        if detector.is_unique_window() {
            break;
        }
    }
    println!("start at char {}", detector.location());
}
