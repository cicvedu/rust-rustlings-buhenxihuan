// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            if first == ' ' {
            
            first.to_string()
        }else{
            format!("{}{}",first.to_uppercase().next().unwrap(), c.as_str())
        }
        },
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let mut string_vec:Vec<String> = Vec::new();
    // for stringOfvec in *words.iter(){
    //     string_vec.push(stringOfvec.next().capitalize_first(stringOfvec));
    // }
    // string_vec
    // let string_vec:Vec<String> = words.iter().map(|&s| capitalize_first(s)).collect();
    // string_vec

    words.iter().map(|&s| capitalize_first(s)).collect()

    
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // let mut sting_append: String = String::new();
    // for string_alone in words.iter().map(|&s| capitalize_first(s)).collect().iter(){
    //     string_append += string_alone.pop();
    // }
   // string_append
    let Vec_string :Vec<String>= words.iter().map(|&s| capitalize_first(s)).collect();
    // let mut return_string = Vec_string.iter();
    // let mut real_string = return_string.next().unwrap().to_string();
    // while return_string {
    //     //println!("{}",return_string.next().unwrap());
    //     real_string += return_string.next().unwrap();
    // }
    // real_string
    Vec_string.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
