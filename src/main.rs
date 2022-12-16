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

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} anmd {:?} is {:?}", stringify!($left), stringify!($right), stringify!($left && $right))
    };
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} os {:?}", stringify!($left), stringify!($right), stringify!($left || $right))
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    say_hello!();

    foo();
    bar();

    print_result!(1u32 + 1);

    // blocks are expressions too
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));

    calculate! {
        eval 1 + 2 // eval is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
