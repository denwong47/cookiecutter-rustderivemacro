//! This library provides a simple derive macro to map Enum variants {{cookiecutter.macro_purpose}}.
//!
//! Example
//! -------
//!
//! ```rust
//! use {{cookiecutter.project_slug}}::{{cookiecutter.macro_name}};
//!
//!
//! #[derive({{cookiecutter.macro_name}})]
//! struct MyStruct {
//!     _populate_me: i32,
//! }
//!
//! assert!(
//!     // Write a test here
//!     true
//! );
//!
//! ```
//!
pub mod prelude;

pub use {{cookiecutter.project_slug}}_macros::*;
pub use {{cookiecutter.project_slug}}_types::*;

mod tests;
