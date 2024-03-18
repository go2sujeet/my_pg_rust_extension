
// pub mod deploy;
use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_my_extension() -> &'static str {
    "Hello, my_extension"
}
// create an extension which will welcome given a name
#[pg_extern]
fn welcome_extension(name: &str) -> String {
    format!("Welcome, {}", name)
}

// crate to just say welcome
#[pg_extern]
fn welcome() -> String {
    "Welcome".to_string()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_my_extension() {
        assert_eq!("Hello, my_extension", crate::hello_my_extension());
    }

    #[pg_test]
    fn test_welcome() {
        assert_eq!("Welcome", crate::welcome()); // Removed the argument from the function call
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
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
