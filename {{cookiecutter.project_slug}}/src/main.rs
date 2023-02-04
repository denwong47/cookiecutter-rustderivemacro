extern crate proc_macro;

use {{cookiecutter.project_slug}}::prelude::*;

#[derive({{cookiecutter.macro_name}})]
pub struct MyStruct {
    _populate_me: i32,
}

fn main() {
    println!("main.rs has run!");
}
