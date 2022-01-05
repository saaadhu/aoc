use std::fs;

fn main() {
    let lines = fs::read_to_string("/home/saaadhu/scratch/input8").expect("Invalid file");

    let mut count = 0;
    for line in lines.split("\n") {
        let digitspart = line.split("|").nth(1).unwrap();
        count += digitspart
            .split(" ")
            .map(|d| d.len() as u64)
            .filter(|x| x == &2 || x == &4 || x == &3 || x == &7)
            .count();
    }

    println! ("{:?}", count);
}
