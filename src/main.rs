fn main() {
    println!("Hello, world!");

    macro_rules! say_hello {
        () => {
            println!("Hello!")
        };
    }

    say_hello!();

    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }

    create_function!(test);
    test();
}
