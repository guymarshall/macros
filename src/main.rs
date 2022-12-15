macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

fn main() {
    say_hello!();

    create_function!(test);
    test();
}
