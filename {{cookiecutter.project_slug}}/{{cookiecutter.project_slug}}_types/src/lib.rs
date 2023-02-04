//! This crate contains traits and structs for export to [`{{cookiecutter.project_slug}}`].
//! 
//! [`{{cookiecutter.project_slug}}`]: ../{{cookiecutter.project_slug}}
//! 

pub mod errors;

mod traits;
pub use traits::*;

mod structs;
pub use structs::*;