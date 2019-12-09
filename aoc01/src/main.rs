use std::io::{prelude::*, BufReader};
use std::fs::File;

fn calculate_fuel_per_module(mass: f64) -> f64 {
    (mass/3.0).floor() - 2.0
}


#[allow(dead_code)]
fn verify_calculate_fuel_per_module() {
    assert_eq!(2.0_f64, calculate_fuel_per_module(12.0));
    assert_eq!(2.0_f64, calculate_fuel_per_module(14.0));
    assert_eq!(654.0_f64, calculate_fuel_per_module(1969.0));
    assert_eq!(33583.0_f64, calculate_fuel_per_module(100756.0));
}

fn part1(line: &std::io::Result<String>) -> f64 {
    let mass : f64 = match line.as_ref().unwrap().trim().parse() {
        Ok(mass) => mass,
        Err(_) => panic!("Can't parse this mass number"),
    };

    calculate_fuel_per_module(mass)
}

fn main() {
    let file = File::open("../inputs/input_day_1").expect("Failed to open file input_day_1");
    let reader = BufReader::new(file);

    let mut sum_part1 : f64 = 0.0;
    let mut sum_part2 : f64 = 0.0;
    
    for line in reader.lines() {
        let fuel = part1(&line);
        let mut subsum = fuel;
        let mut additional_fuel = fuel;

        // calculate additional fuel for the fuel above
        loop {
            additional_fuel = calculate_fuel_per_module(additional_fuel);
            if additional_fuel >= 0.0 {
                subsum += additional_fuel;
            }
            else {
                break;
            }
        }

        sum_part1 += fuel;
        sum_part2 += subsum;
    }

    println!("Part 1: {:?}", sum_part1);
    println!("Part 2: {:?}", sum_part2);
}
