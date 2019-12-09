use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("../inputs/input_day_2").expect("Failed to open file input_day_2");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    let _ = reader.read_line(&mut input);

    let mut output : Vec<u32> = input.trim().split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    //for item in &output {
    for item in output.iter_mut() {
        println!("{:?}", item);
        match item {
            99 => { println!("Output part 1: {:?}", output[0]); break; },
            1 => {
                    let sum = output[item.next()] + output[item.next()];
                    outpupt[item.next()] = sum;
                 },
            2 => {},
            _ => panic!("Incorrect opcode: {:?}", item)
        }

    }
}
