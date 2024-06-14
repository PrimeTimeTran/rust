use std::env;
use std::process;
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        process::exit(1);
    }
    let n: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: The argument must be a positive integer.");
            process::exit(1);
        }
    };
    match fibonacci(n) {
        Some(result) => println!("Fibonacci number {} is {:?}", n, result),
        None => println!(
            "Error: Overflow occurred while calculating Fibonacci number {}",
            n
        ),
    }
}

fn fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    } else if n == 1 {
        return Some(1);
    }

    let mut prev: u64 = 0;
    let mut current: u64 = 1;

    for _ in 2..=n {
        let next = match prev.checked_add(current) {
            Some(value) => value,
            None => return None, // Overflow detected, return None
        };
        prev = current;
        current = next;
    }

    Some(current)
}
