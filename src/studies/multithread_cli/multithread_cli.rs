mod commands;

use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use commands::CliCommand;

fn command_runner(rx: Receiver<CliCommand>) -> u64 {
    loop {
        let cmd = rx.recv().unwrap();

        // Generate a random number to use as a jobid
        let job_id = rand::random::<u64>();

        println!("JOB{job_id}: Received {cmd:?}: Processing...");
        thread::sleep(Duration::from_millis(3000));
        println!("JOB{job_id}: Calculation Done!");

        let (left, signal, right, result) = match cmd {
            CliCommand::Sum(left, right) => (left, "+", right, left + right),
            CliCommand::Minus(left, right) => (left, "-", right, left - right),
            CliCommand::Multiply(left, right) => (left, "*", right, left * right),
            CliCommand::Divide(left, right) => (left, "/", right, left / right),

        };

        // Print the result.
        thread::sleep(Duration::from_millis(1000));
        println!("JOB{job_id}: Result: {} {} {} = {}", left, signal, right, result);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<CliCommand>();

    thread::spawn(move || command_runner(rx));

    loop {
        // Ask for two numbers separated by +, -, *, or /.
        println!("Enter a calculation in the format \"X <op> Y\", where OP=+,-,*,/: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Split input into left operand, signal and right operand, using regular expressions
        let split_re = regex::Regex::new(r"(\d+)\s*([\+\-\*\/])\s*(\d+)").unwrap();

        // If the input matches the regular expression, extract the operands and the signal.
        if let Some(captures) = split_re.captures(input) {
            let left = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let right = captures.get(3).unwrap().as_str().parse::<u64>().unwrap();
            let signal = captures.get(2).unwrap().as_str();

            // Build CliCommand
            let cmd = match signal {
                "+" => CliCommand::Sum(left as i32, right as i32),
                "-" => CliCommand::Minus(left as i32, right as i32),
                "*" => CliCommand::Multiply(left as i32, right as i32),
                "/" => CliCommand::Divide(left as i32, right as i32),
                _ => panic!("Invalid signal!"),
            };

            // Send command over channel to command_runner thread.
            tx.send(cmd).unwrap();

        } else {
            println!("Invalid input!");
        }
    }
}
