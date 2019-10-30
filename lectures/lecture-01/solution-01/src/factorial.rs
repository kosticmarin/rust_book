use std::io::{self, Write};

fn factorial(n: i32) -> Result<i64, String> {
    if n < 0 || n > 20 {
        Err(format!(
            "'{}' is not in the valid range. Try a number from range [0, 20]",
            n
        ))
    } else {
        let mut sum = 1i64;
        for x in 2..=n {
            sum *= x as i64;
        }
        Ok(sum)
    }
}

fn main() {
    let input = io::stdin();
    let mut output = io::stdout();
    loop {
        let mut line = String::new();
        print!("Enter a number >");
        output.flush().expect("Error flushing stdout!");
        input
            .read_line(&mut line)
            .expect("Error reading from stdin!");
        let trimed_line = line.trim().to_lowercase();
        if trimed_line == "exit" {
            break;
        }
        let n: i32 = match trimed_line.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("'{}' is not a valid number.", trimed_line);
                continue;
            }
        };
        match factorial(n) {
            Ok(v) => println!("{}! = {}", n, v),
            Err(e) => println!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_fact_is_err() {
        let result = factorial(0);
        assert!(result.is_ok());
        assert_eq!(1i64, result.unwrap());
    }

    #[test]
    fn one_fact_is_err() {
        let result = factorial(1);
        assert!(result.is_ok());
        assert_eq!(1i64, result.unwrap());
    }

    #[test]
    fn two_fact_is_two() {
        let result = factorial(2);
        assert!(result.is_ok());
        assert_eq!(2i64, result.unwrap());
    }

    #[test]
    fn three_fact_is_six() {
        let result = factorial(3);
        assert!(result.is_ok());
        assert_eq!(6i64, result.unwrap());
    }

    #[test]
    fn twenty_fact() {
        let result = factorial(20);
        assert!(result.is_ok());
        assert_eq!(2432902008176640000i64, result.unwrap());
    }

    #[test]
    fn twenty_one_fact_is_err() {
        assert!(factorial(21).is_err());
    }
}
