use std::io::{stdin, stdout, Write};
use human_panic::{setup_panic};

fn main() {
    setup_panic!();

    println!("Welcome to the brain**** interpreter");
    loop {
        print!("Enter an input: ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let mut pointer: usize = 0;
        let mut memory = [0u8; 30_000];
        if !check_function_bounds(&buffer) {
            eprintln!("Error: Function bounds are not valid. Please check your code.");
            return;
        }
        let mut program_input = String::new();
        if check_input_command(&buffer) {
            print!("Enter the input for the program: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut program_input).unwrap();
        }
        execute_buffer(&buffer, &program_input,&mut memory, &mut pointer);
        println!();
    }
}

fn execute_buffer(buffer: &str, program_input: &str, memory: &mut [u8], pointer: &mut usize) {
    let mut user_input = program_input;
    let mut input = buffer.chars().enumerate();
    loop {
        let command = input.next();
        if command.is_none() {
            break;
        }
        let (idx, command) = command.unwrap();
        match command {
            '[' => {
                let start_idx = idx;
                let mut end_idx = 0;
                for (inner_idx, inner_command) in input.by_ref() {
                    if inner_command == ']' {
                        end_idx = inner_idx;
                        break;
                    }
                }
                let loop_slice = &buffer[start_idx + 1..end_idx];
                while memory[*pointer] != 0 {
                    execute_buffer(loop_slice, user_input, memory, pointer);
                }
            }
            '+' => {
                memory[*pointer] = memory[*pointer].wrapping_add(1);
            }
            '-' => {
                memory[*pointer] = memory[*pointer].wrapping_sub(1);
            }
            '<' => {
                *pointer = {
                    if *pointer == 0 {
                        29_999
                    } else {
                        *pointer - 1
                    }
                }
            }
            '>' => {
                *pointer = {
                    if *pointer == 29_999 {
                        0
                    } else {
                        *pointer + 1
                    }
                }
            }
            '.' => {
                print!("{}", memory[*pointer] as char);
            },
            ',' => {
                let mut input = user_input.chars();
                let char = input.next().unwrap_or('0');
                user_input = input.as_str();
                memory[*pointer] = char as u8;
            }
            _ => {
                return;
            }
        }
    }
}

fn check_function_bounds(buffer: &str) -> bool {
    let mut count = 0;
    for command in buffer.chars() {
        match command {
            '[' => {
                count += 1;
            }
            ']' => {
                count -= 1;
            }
            _ => {
                continue;
            }
        }
    }
    count == 0
}

fn check_input_command(buffer: &str) -> bool {
    buffer.chars().any(|c| c==',')
}