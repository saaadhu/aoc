use std::fs;
use std::collections::HashSet;

fn computedigit(led: &str, combo: &Vec<char>) -> Option<usize> {
    /* Endianness of bits is reversed, so digts are rotated 180.
       Doesn't affect results though. */
    let digits = [0b1110111,
                  0b0010010,
                  0b1011101,
                  0b1011011,
                  0b0111010,
                  0b1101011,
                  0b1101111,
                  0b1010010,
                  0b1111111,
                  0b1111011];

    let mut bits = 0;
    for seg in led.chars() {
        let pos = combo.iter().position(|&x| x == seg).unwrap();
        bits |= 1 << pos;
    }
    return digits.iter().position(|&x| x == bits);
}

fn combinations(orig: HashSet<char>, s: &mut Vec<char>, pos: usize) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    
    if orig.len() == 0 {
        res.push(s.clone());
        return res;
    }

    for c in &orig {
        s[pos] = *c;

        let mut newset = orig.clone();
        newset.remove(&c);
        for r in combinations(newset, s, pos + 1) {
            res.push(r);
        }
    }
    return res;
}

fn main() {
    let lines = fs::read_to_string("/home/saaadhu/scratch/input8").expect("Invalid file");
    let mut input = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let combos = combinations(input.iter().cloned().collect(), &mut input, 0);

    let mut count = 0;
    for line in lines.split("\n") {
        let mut parts = line.split("|");
        let seq = parts.next().unwrap().trim();
        let tocompute = parts.next().unwrap().trim();

        for combo in &combos {
            let mut digits = [0; 10];
            for led in seq.split(" ") {
                let d = computedigit(led, &combo);
                if let Some(digit) = d {
                    digits[digit as usize] = 1;
                }
            }

            if digits.iter().all(|&x| x == 1) {
                let mut res = 0;
                for part in tocompute.split(" ") {
                    let digit = computedigit(&part, &combo).unwrap();
                    res *= 10;
                    res += digit;
                }
                println! ("{:?}, {}", combo, res);
                count += res;
                break;
            }
        }
    }

    println! ("{}", count);
}
