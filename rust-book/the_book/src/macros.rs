pub fn run_macros() {}

// 1. declarative macro
// slightly simplified definition of the vec! macro.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
  }
// } We’ve defined a macro that can take any number of arguments of any type and can generate code to create a vector containing the specified elements.

// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}
