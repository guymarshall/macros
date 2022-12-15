fn main() {
    println!("Hello, world!");

    macro_rules! say_hello {
        () => {
            println!("Hello!")
        };
    }

    say_hello!();
}
