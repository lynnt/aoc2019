use std::io::{prelude::*, BufReader};
use std::fs::File;

fn read_file() -> Vec<usize> {
    let file = File::open("../inputs/input_day_2").expect("Failed to open file input_day_2");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    let _ = reader.read_line(&mut input);

    return input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
}

pub fn part1(output: &mut Vec<usize>, noun: Option<usize>, verb: Option<usize>) {
    output[1] = noun.unwrap_or(output[1]);
    output[2] = verb.unwrap_or(output[2]);

    let mut curr_pos = 0;
    let len = output.len();

    while curr_pos < len {
        let opcode = output[curr_pos];
        match opcode {
            99 => { return; }
            1 | 2  => {
                    if curr_pos + 3 >= len {
                        panic!("Out of range");
                    }

                    let pos1 = output[curr_pos+1];
                    let pos2 = output[curr_pos+2];
                    let pos3 = output[curr_pos+3];

                    match opcode {
                        1 => {
                                output[pos3] = output[pos1] + output[pos2];
                             }
                        2 => {
                                output[pos3] = output[pos1] * output[pos2];
                             }
                        _ => panic!("Unknown opcode: {:?}", opcode)
                    }
                  }
            _  => panic!("Unknown opcode: {:?}", opcode)
        }

        curr_pos += 4;
    }
}

pub fn part2() -> Option<(usize, usize)> {
    let input = read_file();
    let expected_output = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
        let mut output = input.clone();
            part1(&mut output, Some(noun), Some(verb));
            if output[0] == expected_output {
                return Some((noun, verb));
            }
        }
    }

    None
}

fn main() {
    // revert to the state before the machine caught on fire
    let mut output = read_file();
    part1(&mut output, Some(12), Some(2));
    println!("Output for part 1: {:?}", output[0]);

    let (noun, verb) = part2().unwrap();
    println!("Output for part 2: ({}, {}) => Final output: {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn basic_default_test() {
        let mut input : Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected : Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        part1(&mut input, None, None);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test1() {
        let mut input : Vec<usize> = vec![1,0,0,0,99];
        let expected : Vec<usize> = vec![2,0,0,0,99];
        part1(&mut input, None, None);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test2() {
        let mut input : Vec<usize> = vec![2,3,0,3,99];
        let expected : Vec<usize> = vec![2,3,0,6,99];
        part1(&mut input, None, None);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test3() {
        let mut input : Vec<usize> = vec![2,4,4,5,99,0];
        let expected : Vec<usize> = vec![2,4,4,5,99,9801];
        part1(&mut input, None, None);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test4() {
        let mut input : Vec<usize> = vec![1,1,1,4,99,5,6,0,99];
        let expected : Vec<usize> = vec![30,1,1,4,2,5,6,0,99];
        part1(&mut input, None, None);
        assert_eq!(expected, input);
    }
}

