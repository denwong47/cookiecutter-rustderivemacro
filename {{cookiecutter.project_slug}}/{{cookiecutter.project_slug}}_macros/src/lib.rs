extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, DeriveInput};

mod func;

/// A simple derive macro {{cookiecutter.macro_purpose}}.
///
/// Example
/// -------
///
/// ```ignore
/// use {{cookiecutter.project_slug}}::{{cookiecutter.macro_name}};
///
/// #[derive(ValuedEnum)]
/// pub struct MyStruct {
///     _populate_me: i32,
/// }
/// ```
///
/// will Expand into:
///
/// ```ignore
/// use {{cookiecutter.project_slug}}::{{cookiecutter.macro_name}};
///
/// // POPULATE ME
/// ```
#[proc_macro_derive({{cookiecutter.macro_name}}, attributes(__change_me__))]
pub fn {{cookiecutter.project_slug}}(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // =================================================================================
    // Build the output
    let expanded = quote! {
        use {{cookiecutter.project_slug}}_types::{{cookiecutter.macro_name}};
        impl {{cookiecutter.project_slug}}_types::{{cookiecutter.macro_name}} for #name {

            /// Sample function that always return a fixed value.
            fn test_{{cookiecutter.project_slug}}(&self) -> i32 {
                42069
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
