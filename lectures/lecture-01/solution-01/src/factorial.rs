fn factorial(n: i32) -> Result<i64, String> {
    if n < 2 || n > 20 {
        Err(format!(
            "'{}' is not in the valid range. Try a number from range [2, 20]",
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
    println!("Hello, factorial");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_fact_is_err() {
        assert!(factorial(0).is_err());
    }

    #[test]
    fn one_fact_is_err() {
        assert!(factorial(1).is_err());
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
