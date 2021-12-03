use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/saaadhu/scratch/input")
        .expect("Could not read file");
    let mut increases = 0;
    let mut previous = -1;

    for s in contents.split_whitespace() {
        let current = s.parse().expect("Could not convert string to number");
        if previous != -1 && current > previous {
            increases = increases + 1;
        }
        previous = current;
    }
    
    println!("Increases: {}", increases);
}
