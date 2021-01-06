use std::io::stdin;
use std::collections::HashMap;

#[derive(Debug)]
struct Yubi {
    code: String,
    ssn: String,
}

impl Yubi {
    fn new(c: &str, s: &str) -> Self {
        Self {
            code: c.to_string(),
            ssn: s.to_string()
        }
    }
}

fn main() {
    // key_out contains the output from the yubikey when touched
    let key_out = get_yubi_stream();
    let mut my_yubi = Yubi::new("", "");

    match key_out.len() {
        44 => { my_yubi.ssn = modhex_to_hex(&key_out[0..12]);   // 12 bytes id number
                my_yubi.code = modhex_to_hex(&key_out[12..44]); // 32 bytes otp code
        },
        _ => panic!("the yubikey output does not appear to be of a valid length, bailing out!"),
    }

    println!("{:?}\n", my_yubi);
    println!("Binary Representation of the OTP is:");
    for (i, c) in hex_to_bin(&my_yubi.code).chars().enumerate() {
        if (i % 20) == 19 {
            println!("");
        } else {
            print!("{}", c);
        }
    }
    println!("");
}

fn hex_to_bin(s: &str) -> String {

    let mut res_str = String::new();

    // lookup hashtable for hex -> bin
    let h2b : HashMap<char, &str> = [
        ('0', "0000"), ('1', "0001"), ('2', "0010"), ('3', "0011"),
        ('4', "0100"), ('5', "0101"), ('6', "0110"), ('7', "0111"),
        ('8', "1000"), ('9', "1001"), ('a', "1010"), ('A', "1010"),
        ('b', "1011"), ('B', "1011"), ('c', "1100"), ('C', "1100"),
        ('d', "1101"), ('D', "1101"), ('e', "1110"), ('E', "1110"),
        ('f', "1111"), ('F', "1111"),
    ].iter().cloned().collect();

    for (_, c) in s.chars().enumerate() {
        match h2b.get(&c) {
            Some(x) => { res_str.push_str(*x); res_str.push(' ');}
            None => panic!("invalid data to convert, bailing out!"),
        }
    }
    res_str
}

fn modhex_to_hex(s: &str) -> String {

    let mut res_str = String::new();
    let mh2h : HashMap<char, char> = [
        ('c', '0'), ('f', '4'), ('j', '8'), ('r', 'c'),
        ('b', '1'), ('g', '5'), ('k', '9'), ('t', 'd'),
        ('d', '2'), ('h', '6'), ('l', 'a'), ('u', 'e'),
        ('e', '3'), ('i', '7'), ('n', 'b'), ('v', 'f')
    ].iter().cloned().collect();

        for (_, c) in s.chars().enumerate() {
        match mh2h.get(&c) {
            Some(x) => res_str.push(*x),
            None => panic!("Invalid modhex encountered, bailing out!"),
        }
    }
    res_str
}

fn get_yubi_stream() -> String {
    println!("touch your yubikey button now ...");
    let mut yubi_stream = String::new();
    stdin()
        .read_line(&mut yubi_stream)
        .expect("Trouble reading the yubi characters");

    yubi_stream.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levelone() {
        assert_eq!(modhex_to_hex("vutr"), "fedc");
        assert_eq!(modhex_to_hex("jkln"), "89ab");
        assert_eq!(modhex_to_hex("ifhg"), "7465");
        assert_eq!(modhex_to_hex("cebd"), "0312");
    }

    #[test]
    fn levelzero() {
        assert_eq!(hex_to_bin("012AbF"), "0000 0001 0010 1010 1011 1111 ");
    }
}
