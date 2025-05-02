use std::io;
use std::collections::HashMap;

fn main() {
    println!("This program calculates the median and the mode of a list of integers!");

    loop {
        println!("Please input the list of integers, separated by space.");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        let mut input_vec = Vec::new();

        for c in input_str.trim().split_whitespace() {
            let i: i32 = c.parse().expect("Please type a number!");
            input_vec.push(i);
        }

        let input_len: i32 = input_vec.len() as i32;

        input_vec.sort();

        let median: f64 = if input_len == 1 {
            input_vec[0] as f64
        } else if input_len % 2 != 0 {
            let mid_index: i32 = (&input_len - 1) / 2;
            input_vec[mid_index as usize] as f64
        } else if input_len % 2 == 0 {
            let input_len_half: i32 = &input_len / 2;
            let input_len_half_minus: i32 = input_len_half - 1;
            let first: f64 = input_vec[input_len_half as usize] as f64;
            let second: f64 = input_vec[input_len_half_minus as usize] as f64;
            (first + second) / 2.0
        } else {
            println!("No number entered!");
            continue;
        };

        let mut count_map = HashMap::new();

        for num in &input_vec {
            let count = count_map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut modes = Vec::new();
        let mut max: i32 = 0;

        for (_key, value) in &count_map {
            if *value > max {
                max = *value;
            }
        }

        for (key, value) in &count_map {
            if *value == max {
                modes.push(*key);
            }
        }

        println!("Your median: {median}");
        println!("Your mode(s): {:?}", modes);
        break;
    }
}
