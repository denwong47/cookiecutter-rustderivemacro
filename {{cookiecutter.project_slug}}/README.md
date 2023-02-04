# {{cookiecutter.macro_name}} derive macro

![CI Checks](https://github.com/denwong47/{{cookiecutter.project_slug}}/actions/workflows/CI.yml/badge.svg?branch=main)

This library provides a simple derive macro {{cookiecutter.macro_purpose}}.

Example
-------

```rust
use {{cookiecutter.project_slug}}::{{cookiecutter.macro_name}};


#[derive({{cookiecutter.macro_name}})]
struct MyStruct {
    _populate_me: i32,
}

assert!(
    // Write a test here
    true
);

```