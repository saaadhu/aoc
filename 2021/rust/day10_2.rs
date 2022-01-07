use std::fs;
use std::collections::HashMap;

fn is_corrupted(line: &str) -> (bool, Vec<char>) {
    let mut stack : Vec<char> = Vec::new();

    let term : HashMap<char, (char, u32)> = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137))]);

    for c in line.chars() {
        if let Some((m, _)) = term.get(&c) {
            if let Some(s) = stack.pop() {
                if s != *m {
                    return (true, stack);
                }
            }
        } else {
            stack.push(c);
        }
    }

    return (false, stack);
}

fn main() {
    let input = fs::read_to_string("/home/saaadhu/scratch/input10").expect("Invalid file");

    let lines : Vec<&str> = input.split("\n").collect();

    let closers : HashMap<char, u32> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)]);

    let mut scores :Vec<u64> = Vec::new();
    for line in lines {
        let (corrupted, mut stack) = is_corrupted(&line);
        if corrupted {
            continue;
        }

        let mut score: u64 = 0;
        while let Some(c) = stack.pop() {
            let cscore =  closers.get(&c).unwrap();
            score = (score * 5) + *cscore as u64;
            //println! ("{} {} {}", c, cscore, score);
        }

        if score == 0 {
            continue;
        }

        println! ("{}, {}", line, score);
        scores.push(score);
    }

    scores.sort();
    println! ("{:?}", scores[scores.len()/2]);
}

      
