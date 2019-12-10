use std::io::{prelude::*, BufReader};
use std::fs::File;

pub fn solution(output: &mut Vec<usize>) {
    let mut curr_pos = 0;
    let len = output.len();

    while curr_pos < len {
        let opcode = output[curr_pos];
        match opcode {
            99 => { println!("Output part 1: {:?}", output[0]); return; },
            1 | 2  => {
                    if curr_pos + 3 >= len {
                        panic!("Out of range");
                    }

                    let pos1 = output[curr_pos+1];
                    let pos2 = output[curr_pos+2];
                    let pos3 = output[curr_pos+3];

                    match opcode {
                        1 => {
                                let temp = output[pos1] + output[pos2];
                                println!("Add output: A[{:?}] = A[{:?}] + A[{:?}] = {:?} + {:?} = {:?}", pos3, pos1, pos2, output[pos1], output[pos2], temp);
                                output[pos3] = temp;
                                //output[pos3] = output[pos1] + output[pos2];
                             }
                        2 => {
                                let temp = output[pos1] * output[pos2];
                                println!("Multiply output: A[{:?}] = A[{:?}] * A[{:?}] = {:?} * {:?} = {:?}", pos3, pos1, pos2, output[pos1], output[pos2], temp);
                                output[pos3] = temp;
                                //output[pos3] = output[pos1] * output[pos2];
                             }
                        _ => panic!("Unknown opcode: {:?}", opcode)
                    }
                  }
            _  => panic!("Unknown opcode: {:?}", opcode)
        }

        curr_pos += 4;
    }
}

fn main() {
    let file = File::open("../inputs/input_day_2").expect("Failed to open file input_day_2");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    let _ = reader.read_line(&mut input);

    let mut output : Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    println!("Input : {:?}", output);

    println!("");
    solution(&mut output);

    println!("");
    println!("Final output: {:?}", output);

    assert_ne!(234713, output[0]);
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn basic_default_test() {
        let mut input : Vec<usize> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected : Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        solution(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test1() {
        let mut input : Vec<usize> = vec![1,0,0,0,99];
        let expected : Vec<usize> = vec![2,0,0,0,99];
        solution(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test2() {
        let mut input : Vec<usize> = vec![2,3,0,3,99];
        let expected : Vec<usize> = vec![2,3,0,6,99];
        solution(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test3() {
        let mut input : Vec<usize> = vec![2,4,4,5,99,0];
        let expected : Vec<usize> = vec![2,4,4,5,99,9801];
        solution(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn basic_test4() {
        let mut input : Vec<usize> = vec![1,1,1,4,99,5,6,0,99];
        let expected : Vec<usize> = vec![30,1,1,4,2,5,6,0,99];
        solution(&mut input);
        assert_eq!(expected, input);
    }
}

