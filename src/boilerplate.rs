use std::str::Chars;

pub trait CustomInto<T> {
    fn into_float(&self) -> T;
}

impl CustomInto<f32> for String {
    fn into_float(&self) -> f32 {
        let mut item = self.clone();

        // If the number does contain a ., add one to the end
        if !item.contains(".") {
            item.push_str(".0");
        }

        return item.parse().unwrap();
    }
}

pub trait BetterStrings {
    fn remove_first(&mut self);
    fn pop_first(&mut self) -> char;
    fn peek(&self) -> char;
    fn remove_first_if_matches(&mut self, match_case: &str) -> Result<(), String>;
    fn trim_self(&mut self);
    fn trim_string(&self) -> String;
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

    fn trim_self(&mut self) {
        let trimmed = self.trim().to_owned();
        *self = trimmed
    }

    fn trim_string(&self) -> String {
        self.trim().to_owned()
    }
}

pub trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Chars<'_> {
    fn as_string(&self) -> String {
        return self.as_str().to_owned();
    }
}

pub trait CustomMap {
    fn map_values<T: Sized, const COUNT: usize>(&self, values: [(Self, T); COUNT]) -> Option<T>
    where
        Self: Sized;
}

impl CustomMap for char {
    fn map_values<T: Sized, const COUNT: usize>(&self, values: [(Self, T); COUNT]) -> Option<T> {
        for (char_val, out_val) in values {
            if char_val == *self {
                return Some(out_val);
            }
        }

        None
    }
}
