use pgx::*;

mod phone_number;

// Required by all `pgx` extensions.  Indicates to Postgres, when it loads the shared library
// that the library is really a Postgres extension
pg_module_magic!();

#[pg_extern]
fn hello_postgresconf() -> &'static str {
    "Hello, postgresconf"
}

#[pg_extern]
fn sum_array(input: Vec<i64>) -> i64 {
    input.into_iter().sum()
}

#[pg_extern]
fn my_generate_series(
    start: i64,
    end: i64,
    step: default!(i64, 1),
) -> impl std::iter::Iterator<Item = i64> {
    (start..=end).into_iter().step_by(step as usize)
}

#[derive(PostgresEnum)]
pub enum Species {
    Dog,
    Cat,
    Fish,
}

#[pg_extern]
fn set_of_animals() -> impl std::iter::Iterator<
    Item = (
        name!(name, &'static str),
        name!(species, Species),
        name!(age, f32),
    ),
> {
    let names = vec!["Brandy", "Sally", "Anchovy"];
    let species = vec![Species::Dog, Species::Cat, Species::Fish];
    let ages = vec![4.5, 4.0, 3.25];

    names
        .into_iter()
        .zip(species.into_iter())
        .zip(ages.into_iter())
        // need to map the values to convert into a single tuple of three elements
        .map(|((name, species), age)| (name, species, age))
}

#[pg_extern]
fn rust_tuple(name: &str, age: i32) -> (name!(name, &str), name!(age, i32)) {
    (name, age)
}

#[pg_extern]
fn array_of_names() -> Vec<&'static str> {
    vec!["Sally", "Brandy", "Anchovy"]
}

#[pg_extern]
fn array_of_names_with_null() -> Vec<Option<&'static str>> {
    vec![Some("Sally"), None, Some("Brandy"), Some("Anchovy")]
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use crate::array_of_names_with_null;
    use pgx::*;

    #[pg_test]
    fn test_hello_postgresconf() {
        assert_eq!("Hello, postgresconf", crate::hello_postgresconf());
    }

    #[pg_test]
    fn test_generate_series() {
        let result =
            Spi::get_one::<Vec<i64>>("SELECT array_agg(g) FROM my_generate_series(1, 10) g;")
                .expect("SPI result was NULL");
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    }

    #[pg_test]
    fn test_array_of_names_with_null() {
        assert_eq!(
            array_of_names_with_null(),
            vec![Some("Sally"), None, Some("Brandy"), Some("Anchovy")]
        )
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
