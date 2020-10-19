use crate::phone_number::PhoneNumber;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

/// Provides ability to convert a PhoneNumber into a human-readable string representation
/// in the form of: `800-555-1212`
impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // ensure that each part of the phone number is zero-padded
        write!(
            f,
            "{:0width$}-{:0width$}-{:0number_width$}",
            self.area_code,
            self.exchange,
            self.number,
            width = 3,
            number_width = 4
        )
    }
}

/// Possible parsing errors
#[derive(Debug)]
pub enum PhoneNumberParseError {
    InvalidAreaCode,
    InvalidExchange,
    InvalidNumber,
    InvalidFormat,
}

/// Parse a valid PhoneNumber (ie, `800-555-1212`) from a string
///
/// If parsing fails, a `PhoneNumberParseError` variant is returned
impl FromStr for PhoneNumber {
    type Err = PhoneNumberParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn validate<F: FnOnce() -> PhoneNumberParseError>(
            input: Option<&str>,
            digits: usize,
            err_func: F,
        ) -> Result<u16, PhoneNumberParseError> {
            match input {
                // it has the correct number of digits
                Some(value) if value.len() == digits => match value.parse() {
                    // and it parsed into a u16
                    Ok(value) => Ok(value),

                    // it did not parse into a u16
                    Err(_) => Err(err_func()),
                },

                // it does not have the correct number of digits
                Some(_) => Err(err_func()),

                // we don't even have an input value
                None => Err(PhoneNumberParseError::InvalidFormat),
            }
        }

        let mut parts = input.split('-');
        let area_code = validate(parts.next(), 3, || PhoneNumberParseError::InvalidAreaCode)?;
        let exchange = validate(parts.next(), 3, || PhoneNumberParseError::InvalidExchange)?;
        let number = validate(parts.next(), 4, || PhoneNumberParseError::InvalidNumber)?;

        if parts.next().is_some() {
            // we have too many parts
            Err(PhoneNumberParseError::InvalidFormat)
        } else {
            // it's valid per our rules
            Ok(PhoneNumber {
                area_code,
                exchange,
                number,
            })
        }
    }
}
