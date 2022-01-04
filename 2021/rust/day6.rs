use std::fs;

fn main() {
    let ages : Vec<u32> = fs::read_to_string("/home/saaadhu/scratch/input6")
        .expect("Invalid file")
        .split(",")
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    let mut children = [0; 9];
    for age in ages {
        children[age as usize] += 1;
    }

    for _ in 0..256 {
        let orig_children_at_0 = children[0];
        children[0] = 0;
        for i in 1..children.len() {
            children[i-1] = children[i];
        }
        children[6] += orig_children_at_0;
        children[8] = orig_children_at_0;
    }

    println!("{}", children.iter().sum::<u64>());
}
