// https://old.reddit.com/r/dailyprogrammer/comments/akv6z4/20190128_challenge_374_easy_additive_persistence/

// loops you have to do summing its digits until you get a single digit number. Take an integer N:
// Add its digits
// Repeat until the result has 1 digit
// The total number of iterations is the additive persistence of N.

fn additive_persistence(number: u32) -> u32 {
    let mut i = 1;
    let mut digits = number;
    loop {
        digits = digits.to_string().chars().map(|c| c.to_digit(10).expect("should be a digit")).sum();
        if digits < 10 {
            break;
        }
        i += 1;
    }
    return i;
}

fn main() {
    println!("Additive persistence of {} is {}", 32, additive_persistence(32));
    println!("Additive persistence of {} is {}", 199, additive_persistence(199));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_additive_persistence_32() {
        assert_eq!(additive_persistence(32), 1);
    }
  
    #[test]
    fn test_additive_persistence_100() {
        assert_eq!(additive_persistence(100), 1);
    }

    #[test]
    fn test_additive_persistence_199() {
        assert_eq!(additive_persistence(199), 3);
    }    

}