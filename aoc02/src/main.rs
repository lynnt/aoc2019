use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("../inputs/input_day_2").expect("Failed to open file input_day_2");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    let _ = reader.read_line(&mut input);

    let mut output : Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    println!("Input : {:?}", output);
    let len = output.len();

/*
    for (i, element) in output.iter_mut().enumerate() {
        println!("{:?}", element);
        match element {
            99 => { println!("Output part 1: {:?}", output[0]); break; },
            1 => {
                    if i+3 < len {
                        let sum = output[i+1] + output[i+2];
                        output[i+3] = sum;
                    } else {
                        println!("Out of range");
                    }
                    //let sum = output[item.next()] + output[item.next()];
                    //outpupt[item.next()] = sum;
                 },
            2 => {},
            _ => println!("Opcode {:?}", element)//panic!("Incorrect opcode: {:?}", element)
        }
    }
*/
/*
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
*/

    let mut iter = output.iter_mut();

    while let Some(element) = iter.next() {
        println!("{:?}", element);
        match element {
            99 => { println!("Output part 1: {:?}", output[0]); break; },
            1 => {
                    if let Some(pos1) = iter.next() {
                        println!("Value pos1: {:?}", pos1);
                        if let Some(pos2) = iter.next() {
                            println!("Value pos2: {:?}", pos2);
                            if let Some(pos3) = iter.next() {
                                println!("{}", output);
                            }
                        }
                    }
                 },
            2 => {},
            _ => println!("Value: {:?}", element)
        }
    }
}
