# Headache-free environment variables

This crate provides a simple way to define and use environment variables in Rust, wrapping the dotenvy and lazy_static crates. It ensures at first use that all environment variables included in the struct are set and valid, so that you can just run your program without encountering errors at runtime due to missing environment variables.

```rs
/// Define your environment variables in a environment "group"
pub struct Vars {
    pub jwt_secret: String,
    pub manual_cli_mode: bool,
    pub database_url: String,
}

/// vars! is a wrapper around lazy_static! that provides runtime checked variables that are set once, used across the application.
/// It ensures at first use that all required environment variables are set and valid.
superenvy::vars! {
    pub static ref VARS: Vars = Vars {
        jwt_secret: superenvy::var("JWT_SECRET").non_empty().inner,
        manual_cli_mode: superenvy::var_opt("MANUAL_CLI_MODE").map(|v| v.bool()).unwrap_or(false),
        database_url: superenvy::var("DATABASE_URL").non_empty().inner,
    };
}

/// In main.rs:
/// this will now panic if any of the environment variable is not set or empty, we can now be sure that our application will not crash due to missing environment variables.
let _ = VARS.jwt_secret;
```
