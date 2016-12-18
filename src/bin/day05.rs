use std::env;

extern crate md5;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <room_id>", args[0]);
        return;
    }
    let ref door_id:String = args[1];
    println!("Door ID: {}", door_id);

    let mut count = 0;
    let mut pass:Vec<char> = Vec::new();
    let mut pass2:Vec<char> = Vec::with_capacity(8);
    for _ in 0..8 { pass2.push('_') }
    let mut found = 0;

    loop {
        let search = format!("{}{}", door_id, count);
        let hash = format!("{:x}", md5::compute(search));
        if hash[0..5].to_string() == "00000" {

            if pass.len() < 8 {
                let pchar = hash.chars().nth(5).unwrap();
                pass.push(pchar);
            }

            let raw = hash[5..6].to_string();
            let pos = usize::from_str_radix(&raw, 16).unwrap();
            if pos < 8 && pass2[pos] == '_' {
                let pchar2 = hash.chars().nth(6).unwrap();
                pass2[pos] = pchar2;
                found += 1;
            }

            println!("{} - {:?}", hash, pass2);
        }
        count += 1;
        if found >= 8 {
            break;
        }
    }

    let password:String = pass.into_iter().collect();
    println!("The password is: {}", password);
    let password2:String = pass2.into_iter().collect();
    println!("The second password is: {}", password2);
}
