use std::fs;

fn main() {
    let mut ones = [0; 12]; let mut zeros = [0; 12];
    
    let contents = fs::read_to_string("/home/saaadhu/scratch/input3").expect("Invalid file");
    for line in contents.split_whitespace() {
        let mut num = u32::from_str_radix(line, 2).expect("Expected binary number");

        for i in 0..zeros.len() {
            let val = num & 0x1;
            if val != 0 {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
            num >>= 1;
        }
    }

    for i in 0..zeros.len() {
        println!("{}: 0 [{}] 1 [{}]", i, zeros[i], ones[i]);
    }

    let mut gamma = 0;
    for i in 0..zeros.len() {
        if ones[i] > zeros[i] {
            gamma |= 1 << i;
        }
    }
    let epsilon = 0xFFF &(!gamma);
    println!("{:b} * {:b} = {}", gamma, epsilon, gamma * epsilon);
}
