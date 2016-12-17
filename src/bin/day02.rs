use std::io::{self};
use std::io::BufRead;
use std::collections::HashMap;

extern crate aoc16;

#[derive(Debug)]
struct Keypad {
    pos: aoc16::Point,
    buttons: HashMap<aoc16::Point, char>,
}

impl Keypad {

    fn new(layout: &str, start_button: char) -> Keypad {
        let mut button_lookup = HashMap::new();
        let mut pos = aoc16::Point { x: 0, y: 0 };
        let mut start_pos: Option<aoc16::Point> = None;
        for ch in layout.trim().chars() {
            if ch == ' ' {
                continue;
            }
            if ch == '\n' {
                pos.x = 0;
                pos.y += 1;
                continue;
            }
            button_lookup.insert(pos, ch);
            if ch == start_button {
                start_pos = Some(pos.clone())
            }
            pos.x += 1;
        }
        return Keypad {pos: start_pos.unwrap(), buttons: button_lookup}
    }

    fn active_button(&self) -> char {
        return *self.buttons.get(&self.pos).unwrap()
    }

    fn is_valid_pos(&self, pos: aoc16::Point) -> bool {
        if !self.buttons.contains_key(&pos) {
            return false;
        }
        let button = self.buttons.get(&pos).unwrap();
        if *button == '_' {
            return false;
        } else {
            return true;
        }
    }

    fn move_by(&mut self, delta: aoc16::Point) {
        let new_pos = self.pos + delta;
        if self.is_valid_pos(new_pos) == true {
            self.pos = new_pos;
        }
    }

    fn move_down(&mut self) {
        self.move_by(aoc16::Point { x: 0, y: 1 });
    }

    fn move_up(&mut self) {
        self.move_by(aoc16::Point { x: 0, y: -1 });
    }

    fn move_right(&mut self) {
        self.move_by(aoc16::Point { x: 1, y: 0 });
    }

    fn move_left(&mut self) {
        self.move_by(aoc16::Point { x: -1, y: 0 });
    }

}

fn get_button(line: &str, keypad: &mut Keypad) -> char {
    for ch in line.chars() {
        match ch {
            'U' => keypad.move_up(),
            'D' => keypad.move_down(),
            'R' => keypad.move_right(),
            'L' => keypad.move_left(),
            _ => panic!("Invalid keypad instruction: {}", ch),
        }
    }
    return keypad.active_button();
}

fn main() {
    let keypad1_layout = r#"
        1 2 3
        4 5 6
        7 8 9
    "#;
    let mut keypad1 = Keypad::new(keypad1_layout, '5');

    let keypad2_layout = r#"
        _ _ 1 _ _
        _ 2 3 4 _
        5 6 7 8 9
        _ A B C _
        _ _ D _ _
    "#;
    let mut keypad2 = Keypad::new(keypad2_layout, '5');

    let stdin = io::stdin();
    let mut code1_parts = Vec::new();
    let mut code2_parts = Vec::new();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        code1_parts.push(get_button(&unwrapped, &mut keypad1));
        code2_parts.push(get_button(&unwrapped, &mut keypad2));
    }

    let code1: String = code1_parts.into_iter().collect();
    let code2: String = code2_parts.into_iter().collect();
    println!("Code 1: {}", code1);
    println!("Code 2: {}", code2);
}
