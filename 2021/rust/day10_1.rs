use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input10").expect("Invalid file");
    let term : HashMap<char, (char, u32)> = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137))]);

    let lines : Vec<&str> = input.split("\n").collect();

    let mut points = 0;
    for line in lines {
        let mut stack : Vec<char> = Vec::new();
        for c in line.chars() {
            if let Some((m, score)) = term.get(&c) {
                if let Some(s) = stack.pop() {
                    if s != *m {
                        points += score;
                    }
                }
            } else {
                stack.push(c);
            }
        }
    }

    println! ("{}", points);
}
