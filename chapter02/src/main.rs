macro_rules! ignore {
        ($_:expr) => {};
    }

fn main() {


    /// Variables in rust are immutable by default.
    /// declaring a variable uses the following syntax:
    /// let name = expression;
    /// declaring a variable as mutable uses the mut keyword
    /// let mut name = expression;
    /// variables can not be declared at global scope.

    // this macro deletes this code block so the program can compile.
    ignore!(
    // compile error
    // This is an example of trying to mutate a variable,
    // that was not declared mutable.
    {
        let x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    });
    // x in not marked as mutable,
    // and is not being declared as a shadow variable

/*
    $ cargo run
       Compiling variables v0.1.0 (file:///projects/variables)
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src/main.rs:4:5
      |
    2 |     let x = 5;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    3 |     println!("The value of x is: {x}");
    4 |     x = 6;
      |     ^^^^^ cannot assign twice to immutable variable

    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `variables` due to previous error
*/

    {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    // $ cargo run
    //     Compiling variables v0.1.0 (file:///projects/variables)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.30s
    //       Running `target/debug/variables`
    // The value of x is: 5
    // The value of x is: 6

    /// Constants exist in rust and are declared with the following syntax:
    /// const name: type = expression;
    /// variables support optional type annotation, but
    /// constants require type annotation.
    /// constants can be declared at global scope but must
    /// be computable at compile time.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    /// Rust allows variables to be shadowed.
    /// this means that a previously used variable name can be assigned to a new value
    /// even if that variable was not marked as mut or it was a different type.
    /// this is useful for typecasting a value as a new name is not required.

    {   // this is an example of shadowing
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }

    // $ cargo run
    //     Compiling variables v0.1.0 (file:///projects/variables)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.31s
    //       Running `target/debug/variables`
    // The value of x in the inner scope is: 12
    // The value of x is: 6

    {   // this is an example of using shadowing for type casting
        let spaces = "   ";
        let spaces = spaces.len();
    }

    ignore!{
    {   // This is trying to use mut.
        let mut spaces = "   ";
        spaces = spaces.len();
        // In rust you can not change the type of a variable by mutating it.
    }};

    // $ cargo run
    //     Compiling variables v0.1.0 (file:///projects/variables)
    // error[E0308]: mismatched types
    //  --> src/main.rs:3:14
    //   |
    // 2 |     let mut spaces = "   ";
    //   |                      ----- expected due to this value
    // 3 |     spaces = spaces.len();
    //   |              ^^^^^^^^^^^^ expected `&str`, found `usize`
    //
    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `variables` due to previous error


    /// Variables do sometimes require type annotation.
    /// When assigning an ambiguously typed expression rust will
    /// require type annotations.
    /// Because the parse function can many types we must tell rust
    /// what we expect our return value to be.
    /// The parse function actually uses this information to return us
    /// the correct value.

    // The parse function is very useful for type casting that can fail,
    // such as casting a string to an int.
    let guess: u32 = "42".parse().expect("Not a number!");

    // this is a table of integer types in rust
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // floating point numbers in rust
    {
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32
    }


    {   // basic numeric operations
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;

        // left shift
        let four = 0b1 << 2;

        // right shift
        let two = 0b100 >> 1;
    }

    {   // booleans in rust
        let t = true;

        let f: bool = false; // with explicit type annotation
    }



    {

    }

}
