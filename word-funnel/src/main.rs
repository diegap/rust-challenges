// Given two strings of letters, determine whether the second can be made from the first by removing one letter. The remaining letters must stay in the same order.
// https://old.reddit.com/r/dailyprogrammer/comments/98ufvz/20180820_challenge_366_easy_word_funnel_1/

fn main(){
    println!("Hello includes Hell >>> {}", includes("Hello", "Hell"));
    println!("+ + + Run: cargo test to execute the different scenarios");
}

fn includes(word: &str, another_word: &str) -> bool {

    if word.len() <= another_word.len() {
        return false;
    }

    let mut i = 1;

    loop {
        let mut current_word = String::from("");
        current_word.push_str(&word[0..i-1]);
        current_word.push_str(&word[i..word.len()]);
       
        if current_word == another_word {
            return true;
        }
            
        i += 1;
    
        if i > word.len() {
            break;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_includes_word_after_first_letter() {
        assert_eq!(includes("leave","eave"), true);
    }

    #[test]
    fn test_includes_word_after_middle_letter() {
        assert_eq!(includes("reset","rest"), true);
    }
        
    #[test]
    fn test_includes_word_removing_duplicated_letter() {
       assert_eq!(includes("dragoon", "dragon"), true);
    }

    #[test]
    fn test_does_not_include_smaller_word() {
       assert_eq!(includes("eave", "leave"), false);
    }

    #[test]
    fn test_does_not_include_inverted_word() {
       assert_eq!(includes("sleet", "lets"), false);
    }

    #[test]
    fn test_does_not_include_word_removing_one_single_letter() {
       assert_eq!(includes("skiff", "ski"), false);
    }

}
