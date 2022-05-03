/// This file contains some boring string manipulation functions, amongst other
/// things. The point of it is that all of the messy code should be put in here
/// to make the main code more readable
use std::str::Chars;

/// Custom version of into that allows for me to implement the trait on
/// predefined and types like strings. This entire thing is a bit of a hack
pub trait CustomInto<T> {
    /// Converts from one type to another. See specific implementation within
    /// `boilerplate.rs` for details.
    fn convert(&self) -> T;
}

impl CustomInto<f32> for String {
    /// Converts a provided string into a 32 bit floating point number. Note:
    /// there is no error handling, so the code will panic if it crashes.
    fn convert(&self) -> f32 {
        let mut item = self.clone();

        // If the number does contain a ., add one to the end. This is to get
        // around some of the expectations of the float parser
        if !item.contains(".") {
            item.push_str(".0");
        }

        return item.parse().unwrap();
    }
}

/// A number of traits for strings that make writing a parser a touch easier.
/// These wouldn't be required if I wrote a tokenizer, but their hacky
/// implementation requires less code than a tokenizer
pub trait BetterStrings {
    /// Removes the first character from a string
    fn remove_first(&mut self);
    /// Removes the first value from a string and return it's value
    fn pop_first(&mut self) -> char;
    /// Return the first character from a string. If you are using this in
    /// combination with `remove_first`, please use `pop_first`
    fn peek(&self) -> char;
    /// Removes the first character from a string if it matches. Returns an
    /// error if there is no matching first character
    fn remove_first_if_matches(&mut self, match_case: &str) -> Result<(), String>;
    /// Removes excess spaces from the beginning and end of a string, returning
    /// a string as a result
    fn trim_string(&self) -> String;
    /// Removes any spaces from the current string, allowing for easier parsing.
    /// Again, this is a sideeffect of not having a tokenizer
    fn remove_whitespace(self) -> Self;
}

impl BetterStrings for String {
    fn remove_first(&mut self) {
        self.remove(0);
    }

    fn pop_first(&mut self) -> char {
        let first = self.peek();
        self.remove_first();
        first
    }

    fn peek(&self) -> char {
        self.chars().next().unwrap()
    }

    fn remove_first_if_matches(&mut self, match_case: &str) -> Result<(), String> {
        if !self.starts_with(match_case) {
            return Err(format!("Expected {}, found {}", match_case, self.peek()));
        }

        self.remove_first();
        Ok(())
    }

    fn trim_string(&self) -> String {
        self.trim().to_owned()
    }

    fn remove_whitespace(self) -> Self {
        let words: Vec<&str> = self.split_whitespace().collect();
        words.join("")
    }
}

/// Trait to convert any data type to a string
pub trait AsString {
    /// Converts the current datatype into a string  
    fn as_string(&self) -> String;
}

impl AsString for Chars<'_> {
    fn as_string(&self) -> String {
        return self.as_str().to_owned();
    }
}
