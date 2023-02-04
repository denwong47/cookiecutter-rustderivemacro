### Cookiecutter template for
# Rust Derive Macro

![CI Checks](https://github.com/denwong47/cookiecutter-rustderivemacro/actions/workflows/CI.yml/badge.svg?branch=main)

A cookiecutter template for creating Rust Derive Macros efficiently.

## Sample
A [sample repository](https://github.com/denwong47/demo_cookiecutter_rustderivemacro) crated by this template can be previewed,
along with its [Github pages](https://denwong47.github.io/demo_cookiecutter_rustderivemacro).

## First steps
This template is based on cookiecutter, a templating tool in Python. With Python3 installed, you can:

```sh
python3 -m pip install cookiecutter pre-commit

cookiecutter ./cookiecutter-rustderivemacro
```

The template will then prompt for parameters such as git username, project slug and macro name etc.

A new folder will be formed with the name being the chosen project slug.

Assuming the project slug is `my_crate`, you can then:

```sh
cd my_crate
make setup
```

This will set up pre-commit hooks as well as initialise git for you.


## Structure
A `proc_macro` crate cannot export anything alongside it; however it is common for
derive macros to implement custom `trait`s. The solution is to split the crate in to 3:

- a `types` sub-crate to define any errors, enums, structs and traits required by the
  macro.
- a `macro` sub-crate dedicated to exporting the derive macro, which depends on the
  `types` sibling crate, importing it by path.
- a top level crate containing both of above, re-exporting everything they expose.

Using the above example of `my_crate`, the generated will thus take the
form of:

- `my_crate` top level crate, and
    - `my_crate_macros`
    - `my_crate_types` subdirectories.
