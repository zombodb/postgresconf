use pgx::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use pgx::cstr_core::CStr;

mod implementation;

/// standard Rust equality/comparison derives
#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]

/// Support using this struct as a Postgres type, which the easy way requires Serde
#[derive(PostgresType, Serialize, Deserialize)]

/// automatically generate =, <> SQL operator functions
#[derive(PostgresEq)]

/// automatically generate <, >, <=, >=, and "_cmp" SQL functions
/// When "PostgresEq" is also derived, pgx also creates an "opclass" (and family)
/// so that the type can be used in indexes `USING btree`
#[derive(PostgresOrd)]

/// automatically generate a "_hash" function, and the necessary "opclass" (and family)
/// so the type can also be used in indexes `USING hash`
#[derive(PostgresHash)]

/// for denoting we want to manually implement textual input/output functions.  Without this
/// `pgx` will automatically use JSON
#[inoutfuncs]
pub struct PhoneNumber {
    area_code: u16,
    exchange: u16,
    number: u16,
}

/// Custom text input/output functions to provide the ability to textually represent
/// a PhoneNumber in the standard format
impl InOutFuncs for PhoneNumber {
    /// uses the `FromStr` trait implementation for PhoneNumber to parse one given to us by Postgres
    ///
    /// THis will raise an ERROR if the provided text value isn't in the proper format
    fn input(input: &CStr) -> Self
    where
        Self: Sized,
    {
        // convert the input, which is a standard C-string into a Rust &str
        input
            .to_str()
            // Rust only supports properly encoded UTF8 strings
            .expect("input is not valid UTF8")
            // and parse it as a PhoneNumber
            .parse()
            // raising an error if the input is in the wrong format
            .expect("invalid phone number")
    }

    /// Use the `Display` trait implementation to represent a PhoneNumber as text
    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{}", self))
    }
}

#[pg_extern]
fn random_phone_number() -> PhoneNumber {
    PhoneNumber {
        area_code: rand::thread_rng().gen_range(1, 999),
        exchange: rand::thread_rng().gen_range(0, 999),
        number: rand::thread_rng().gen_range(0, 9999),
    }
}
