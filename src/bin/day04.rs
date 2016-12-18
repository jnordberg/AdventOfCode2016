use regex::Regex;
use std::io::BufRead;
use std::io::{self};
use std::collections::HashMap;
use std::cmp::Ordering;

extern crate regex;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: i32,
    checksum: String
}

fn room_sort(a: &(&char, &i16), b: &(&char, &i16)) -> Ordering {
    let count_order = b.1.cmp(a.1);
    if count_order == Ordering::Equal {
        return a.0.cmp(b.0);
    } else {
        return count_order;
    }
}

impl Room {

    fn is_valid(&self) -> bool {
        let mut char_map: HashMap<char, i16> = HashMap::new();
        for ch in self.encrypted_name.chars() {
            if ch == '-' {
                continue;
            }
            *char_map.entry(ch).or_insert(0) += 1;
        }
        let mut ranked_chars:Vec<(&char, &i16)> = char_map.iter().collect();
        ranked_chars.sort_by(room_sort);
        let checksum:String = ranked_chars[0..5].to_vec().into_iter().map(|(ch, _)| *ch).collect();
        return checksum == self.checksum;
    }

    fn decrypt(&self) -> String {
        let mut chars:Vec<char> = Vec::new();
        for ch in self.encrypted_name.chars() {
            if ch == '-' {
                chars.push(' ');
                continue;
            }
            let mut c = ch as u8;
            c -= 97;
            for _ in 0..self.sector_id {
                c = (c + 1) % 26;
            }
            c += 97;
            chars.push(c as char);
        }
        let rv:String = chars.into_iter().collect();
        return rv;
    }

}


fn main() {
    let stdin = io::stdin();

    let re = Regex::new(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]").unwrap();

    let mut sector_sum = 0;

    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let caps = re.captures(&line).unwrap();

        let encrypted_name = String::from(caps.at(1).unwrap());
        let sector_id = caps.at(2).unwrap().parse::<i32>().unwrap();
        let checksum = String::from(caps.at(3).unwrap());

        let room = Room {encrypted_name: encrypted_name, sector_id: sector_id, checksum: checksum};

        if room.is_valid() {
            sector_sum += room.sector_id;
            if room.decrypt() == "northpole object storage" {
                println!("North pole object storage in sector: {}", room.sector_id);
            }
        }
    }

    println!("The sum of all valid room sectors is: {}", sector_sum);
}
