///
///
extern crate proc_macro;

macro_rules! test_factory {
    (
        $name:ident
    ) => {
        #[cfg(test)]
        mod $name {
            extern crate proc_macro;
            use crate::prelude::*;

            #[derive({{cookiecutter.macro_name}})]
            struct MyStruct{
                _populate_me: i32,
            }

            #[test]
            fn test_values() {
                assert_eq!(
                    MyStruct { _populate_me: 0 }.test_{{cookiecutter.project_slug}}(),
                    42069,
                )
            }
        }
    };
}

test_factory!(test_sample);
