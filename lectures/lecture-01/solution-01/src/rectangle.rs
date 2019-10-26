use std::env;
use std::fmt;
use std::io::{self, Stdin, Stdout, Write};

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn perimiter(&self) -> f64 {
        (self.height + self.width) * 2f64
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle width={} and height={} has perimiter={} and area={}.",
            self.width,
            self.height,
            self.perimiter(),
            self.area()
        )
    }
}

fn parse_numbers(args: Vec<String>) -> Result<(f64, f64), String> {
    let mut numbers: Vec<f64> = args
        .into_iter()
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect();
    if numbers.len() != 2 {
        return Err(format!("Invalid input arguments not positive numbers"));
    }
    let height = numbers.pop().unwrap();
    let width = numbers.pop().unwrap();
    Ok((width, height))
}

fn parse_positive_f64_from_stdin(
    input: &Stdin,
    output: &mut Stdout,
    parameter: &str,
) -> Result<f64, String> {
    let mut line = String::new();
    print!("Enter Rectangle {} > ", parameter);
    output.flush().expect("Broken stdout");
    input.read_line(&mut line).expect("Broken stdin");
    let trimmed_line = line.trim().to_lowercase();
    let number = match trimmed_line.parse::<f64>() {
        Ok(v) => v,
        Err(_) => return Err(format!("'{}' not a valid positive number", trimmed_line)),
    };
    if number < 0f64 {
        return Err(format!("{} not a postitive number", number));
    }
    Ok(number)
}

fn execute_from_stdin() {
    let input = io::stdin();
    let mut output = io::stdout();
    loop {
        let width = match parse_positive_f64_from_stdin(&input, &mut output, "width") {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let height = match parse_positive_f64_from_stdin(&input, &mut output, "height") {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let rectangle = Rectangle { width, height };
        println!("{}", rectangle);
        break;
    }
}

fn execute_from_args(args: Vec<String>) {
    let (width, height) = match parse_numbers(args) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let rectangle = Rectangle { width, height };
    println!("{}", rectangle);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number_of_args = args.len();
    if number_of_args == 1 {
        execute_from_stdin();
    } else if number_of_args == 3 {
        execute_from_args(args);
    } else {
        println!(
            "Invalid number of command line arguments {}",
            number_of_args
        );
    }
}
