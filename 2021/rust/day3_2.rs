use std::fs;

fn compute_ones_zeros (v: &Vec<u32>) -> ([i32; 12], [i32; 12]) {
    let mut ones = [0; 12]; let mut zeros = [0; 12];

    for num in v {
        let mut mask = 0x1;
        for i in 0..zeros.len() {
            let val = num & mask;
            if val != 0 {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
            mask <<= 1;
        }
    }
    return (ones, zeros);
}

fn main() {
    let mut input : Vec<u32> = Vec::new();
    
    let contents = fs::read_to_string("/home/saaadhu/scratch/input3").expect("Invalid file");
    for line in contents.split_whitespace() {
        let num = u32::from_str_radix(line, 2).expect("Expected binary number");
        input.push(num);
    }

    let mut oxyrating = 0;
    let mut corating  = 0;

    let mut oxyinput = input.clone();
    let mut coinput = input.clone();

    for i in (0..12).rev() {
        let oxyratingbit : u32;
        let (ones,zeros) = compute_ones_zeros (&oxyinput);
        if ones[i] > zeros[i] {
            oxyratingbit = 1;
        } else if zeros[i] > ones[i] {
            oxyratingbit = 0;
        } else {
            oxyratingbit = 1;
        }

        let (ones,zeros) = compute_ones_zeros (&coinput);
        let coratingbit: u32;

        if ones[i] < zeros[i] {
            coratingbit = 1;
        } else if zeros[i] < ones[i] {
            coratingbit = 0;
        } else {
            coratingbit = 0;
        }

        if oxyrating == 0 {
            oxyinput.retain (|num| { num & (1 << i) == oxyratingbit << i });


            if oxyinput.len() == 1 {
                oxyrating = oxyinput[0];
            }
        }

        if corating == 0 {
            coinput.retain(|num| { num & (1 << i) == coratingbit << i });

            if coinput.len() == 1 {
                corating = coinput[0];
            }
        }
    }
    println!("{} * {} = {}", oxyrating, corating, oxyrating * corating);
}
