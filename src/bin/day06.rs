
use std::io::BufRead;
use std::io::{self};
use std::collections::HashMap;

#[derive(Debug)]
struct CharCount {
    counters: HashMap<usize, HashMap<char, i16>>
}

impl CharCount {

    fn new() -> CharCount {
        return CharCount { counters: HashMap::new() }
    }

    fn get_counter(&mut self, pos: usize) -> &mut HashMap<char, i16> {
        return self.counters.entry(pos).or_insert(HashMap::new())
    }

    fn count(&mut self, ch: char, pos: usize) {
        *self.get_counter(pos).entry(ch).or_insert(0) += 1;
    }

    fn get_top(&self) -> Vec<char> {
        let mut rv: Vec<char> = Vec::new();
        let mut sorted_counters: Vec<_> = self.counters.iter().collect();
        sorted_counters.sort_by(|a, b| a.0.cmp(b.0));
        for counter in sorted_counters.iter().map(|v| v.1) {
            let mut chars:Vec<(&char, &i16)> = counter.iter().collect();
            chars.sort_by(|a, b| b.1.cmp(a.1));
            rv.push(*chars[0].0);
        }
        return rv;
    }

    fn get_bottom(&self) -> Vec<char> {
        let mut rv: Vec<char> = Vec::new();
        let mut sorted_counters: Vec<_> = self.counters.iter().collect();
        sorted_counters.sort_by(|a, b| a.0.cmp(b.0));
        for counter in sorted_counters.iter().map(|v| v.1) {
            let mut chars:Vec<(&char, &i16)> = counter.iter().collect();
            chars.sort_by(|a, b| a.1.cmp(b.1));
            rv.push(*chars[0].0);
        }
        return rv;
    }

}

fn main() {
    let mut counter = CharCount::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        for (idx, ch) in line.chars().enumerate() {
            counter.count(ch, idx);
        }
    }

    let message:String = counter.get_top().into_iter().collect();
    println!("The message is: {}", message);
    let message2:String = counter.get_bottom().into_iter().collect();
    println!("The decoded message is: {}", message2);
}
