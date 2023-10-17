// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl std::fmt::Display for ParsePersonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsePersonError::Empty => write!(f, "Empty input string"),
            ParsePersonError::BadLen => write!(f, "Incorrect number of fields"),
            ParsePersonError::NoName => write!(f, "Empty name field"),
            ParsePersonError::ParseInt(e) => write!(f, "Error parsing age: {}", e),
        }
    }
}

impl std::error::Error for ParsePersonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParsePersonError::ParseInt(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. If the length of the provided string is 0, an error should be returned
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 2. Split the given string on the commas present in it
        let parts: Vec<&str> = s.split(',').collect();

        // 3. Only 2 elements should be returned from the split, otherwise return an error
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0];
        let age_str = parts[1];

        // 4. Extract the first element from the split operation and use it as the name
        // 5. Extract the other element from the split operation and parse it into a `usize` as the age
        let age = age_str.parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        // 6. If while extracting the name and the age something goes wrong, an error should be returned
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}


fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
