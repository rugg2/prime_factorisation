use std::io;
use std::vec::Vec;

fn main() {
    // This algorithm takes in a positive integer from stdin
    //  and outputs its prime factors in stdout
    // Warning: this is a naive suboptimal algorithm, just for training purposes
    println!("Enter number to factorise into prime numbers");

    loop{
        println!("\nInput number to have it factorised into prime factors");
        println!("(Quit by typing any non numerical character)");

        let input:u32 = get_input_integer();
        let factors:Vec<u32> = prime_factorise(input);
        display_factors(factors);
    }
}

fn get_input_integer()->u32{
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    let input = input_string.trim().parse().expect("Exit - program runs only on positive integers smaller than 2^32-1");
    assert!(input>=1, "Program designed only for strictly positive integers");

    input
}

fn prime_factorise(n: u32) -> Vec<u32>{
    let (prime_factor, remainder) = get_first_prime_factor(n);

    // n is prime
    if prime_factor == 1 {return vec![n]}

    // n not prime
    let mut factors = vec![prime_factor];
    factors.extend(prime_factorise(remainder));
    factors
}

fn get_first_prime_factor(n:u32) -> (u32, u32){
    let root_floor = (n as f64).sqrt() as u32;
    println!("integer square root of {} is {}", n, root_floor);

    let mut prime: u32 = 1;
    let mut remainder: u32 = n;

    for i in 2..root_floor+1 {
        if n % i == 0 {
            prime = i; // we de facto know i is prime
            remainder = n/i;
            break;
        }
    }
    (prime, remainder)
}

fn display_factors(factors:Vec<u32>){
    println!("Factors: ");
        for i in factors{
            print!("{}, ", i);
        }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_7() {
        let factors = prime_factorise(7);
        assert_eq!(factors, [7])
    }

    #[test]
    fn test_2_small_prime_factors() {
        let n = 2*7;
        let factors = prime_factorise(n);
        assert_eq!(factors, [2, 7])
    }

    #[test]
    fn test_2_identical_prime_factors() {
        let n = 3*3;
        let factors = prime_factorise(n);
        assert_eq!(factors, [3, 3])
    }

    #[test]
    fn test_2_large_prime_factors() {
        let n = 91291*34183;
        let factors = prime_factorise(n);
        assert_eq!(factors, [34183, 91291]);
    }

    #[test]
    fn test_5_prime_factors() {
        let n = 2*2*3*7*11;
        let factors = prime_factorise(n);
        assert_eq!(factors, [2, 2, 3, 7, 11]);
    }
}
