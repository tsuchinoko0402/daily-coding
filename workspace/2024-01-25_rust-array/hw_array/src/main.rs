const STRINGS: [&str; 12] = ["H", "e", "l", "l", "o", ",", "W", "o", "r", "l", "d", "!"];
const CHARS: [char;12] = ['H', 'e', 'l', 'l', 'o', ',', 'W', 'o', 'r', 'l', 'd', '!'];

fn main() {
    for s in STRINGS {
        print!("{}", s)
    }
    for c in CHARS {
        print!("{}", c)
    }
}
