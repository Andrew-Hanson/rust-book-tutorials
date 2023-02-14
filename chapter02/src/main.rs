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

    ignore! {
    {   // This is trying to use mut.
        let mut spaces = "   ";
        spaces = spaces.len();
        // In rust you can not change the type of a variable by mutating it.
    }}
    ;

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

    /// characters in rust are 32 bit or made of 4 8 bit bytes.
    /// rust encodes text in UTF8 so many kinds of
    /// character can be used
    {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }

    /// tuples can contain values of different types however they
    /// can not change size.
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }


    /// tuples can be destructured or pattern matched.
    {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {y}");
    }

    /// tuples can also be accessed by index
    {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }
    // This is different than the [] indexing syntax
    // because rust makes certain promises about indexing
    // with that syntax that tuples do not follow.
    // specifically sequences indexed with [] syntax need
    // all elements to be the same size

    /// Rust has arrays which can not change size or contain
    /// values of different types.
    {
        let a = [1, 2, 3, 4, 5];
    }

    // arrays can be useful for collections of constant values.
    {
        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
    }

    // this is an example with type annotation
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        // here 5 is the seize of the array
    }

    // this is an example on initializing an array
    // with elements that all have the same value.
    {
        let a = [3; 5];
        // here 5 is the size of the array
        // 3 is the value each element will be set to
    }

    /// Arrays can be accessed with [] syntax.
    {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }

    // Here is an example of using user input to access an array.
    // If the user inputs an index of outside the range of the
    // array, then the program will panic at runtime.
    // It would be better to use something like an enum so that
    // the logic can be checked at compiletime using
    // rust's exhaustive matching.
    {
        use std::io;

        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}")
    }
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // This is an example of defining and calling a function.
    {
        println!("Hello, world!");
        another_function();

        fn another_function() {
            println!("Another function.");
        }
    }

    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.28s
    //       Running `target/debug/functions`
    // Hello, world!
    // Another function.

    // an example using a parameter (x = 5).
    {
        another_function(5);

        fn another_function(x: i32) {
            println!("The value of x is: {x}");
            // This is an example of string interpolation in rust.
        }
    }

    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    //      Finished dev [unoptimized + debuginfo] target(s) in 1.21s
    //       Running `target/debug/functions`
    // The value of x is: 5

    // Here is an example using two parameters.
    {
        print_labeled_measurement(5, 'h');

        fn print_labeled_measurement(value: i32, unit_label: char) {
            println!("The measurement is: {value}{unit_label}");
        }
    }

    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.31s
    //       Running `target/debug/functions`
    // The measurement is: 5h

    /// In rust semicolons indicate statements.
    /// Lines or blocks of code without semicolons
    /// are expressions that evaluate to some value.
    {
        let y = 6;
        // this will return no value
    }


    ignore! {
    {
        let x = (let y = 6);
        // This will cause a compile error because
        // assigning a variable is always a statement
        // so, (let y = 6) does not return any value.
    }}
    ;

    // $ cargo run
    // Compiling functions v0.1.0 (file:///projects/functions)
    //                                 error: expected expression, found `let` statement
    //     --> src/main.rs:2:14
    //     |
    //     2 |     let x = (let y = 6);
    // |              ^^^
    //
    // error: expected expression, found statement (`let`)
    // --> src/main.rs:2:14
    //     |
    //     2 |     let x = (let y = 6);
    // |              ^^^^^^^^^
    // |
    // = note: variable declaration using `let` is a statement
    //
    // error[E0658]: `let` expressions in this position are unstable
    //     --> src/main.rs:2:14
    //     |
    //     2 |     let x = (let y = 6);
    // |              ^^^^^^^^^
    // |
    // = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    //
    //     warning: unnecessary parentheses around assigned value
    //     --> src/main.rs:2:13
    //     |
    //     2 |     let x = (let y = 6);
    // |             ^         ^
    // |
    // = note: `#[warn(unused_parens)]` on by default
    // help: remove these parentheses
    //     |
    //     2 -     let x = (let y = 6);
    // 2 +     let x = let y = 6;
    // |
    //
    // For more information about this error, try `rustc --explain E0658`.
    // warning: `functions` (bin "functions") generated 1 warning
    // error: could not compile `functions` due to 3 previous errors; 1 warning emitted


    /// In rust, blocks are expressions that return
    /// the last expression in the block.
    {
        let y = {
            let x = 3;

            x + 1
            // This block will evaluate to x + 1
            // which can be assigned to the variable y.
            // In this case y will be 4.
        };

        println!("The value of y is: {y}");
    }   // This block will return nothing because println
    // is followed by a semicolon.

    // This is an example of a function call returning a value
    {
        fn five() -> i32 {
            5
        }  // this function returns 5

        let x = five();

        println!("The value of x is: {x}");
    }

    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.30s
    //       Running `target/debug/functions`
    // The value of x is: 5


    {   // Here is another example of a function returning a value
        let x = plus_one(5);

        println!("The value of x is: {x}");

        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        // In this case println will print 6.
    }

    ignore!{
    {
        let x = plus_one(5);

        println!("The value of x is: {x}");

        fn plus_one(x: i32) -> i32 {
            x + 1;
            // this function will return nothing
            // this is a compile error because the function
            // declares it will return an i32.
        }
    }};

    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    //     error[E0308]: mismatched types
    //     --> src/main.rs:7:24
    //     |
    //   7 | fn plus_one(x: i32) -> i32 {
    //     |    --------            ^^^ expected `i32`, found `()`
    //     |
    //     |
    //     |    implicitly returns `()` as its body has no tail or `return` expression
    //   8 |     x + 1;
    //     |          - help: remove this semicolon
    //
    //     For more information about this error, try `rustc --explain E0308`.
    //     error: could not compile `functions` due to previous error


    /// ----- comments ----------

    // this is a rust comment

    /* this is a block comment */

    /// this is a doc comment
    // Rust allows tests to be inserted in doc comments and run.
    // This means that any examples used in documentation
    // will always be valid!

    // So we’re doing something complicated here, long enough that we need
    // multiple lines of comments to do it! Whew! Hopefully, this comment will
    // explain what’s going on.
    // This is a multi line comment.

    {
        let lucky_number = 7; // I’m feeling lucky today
        // this is also a valid comment
    }

    /// ------ control flow ---------

    {   // here is an example of the if statement
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    // $ cargo run
    //     Compiling branches v0.1.0 (file:///projects/branches)
    //      Finished dev [unoptimized + debuginfo] target(s) in 0.31s
    //       Running `target/debug/branches`
    // condition was true

    {   // This time number is seven.
        let number = 7;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        // $ cargo run
        //     Compiling branches v0.1.0 (file:///projects/branches)
        //      Finished dev [unoptimized + debuginfo] target(s) in 0.31s
        //       Running `target/debug/branches`
        // condition was false
    }

    ignore!{
    {
        let number = 3;

        // This is a compile error
        // Only bools can be used for if statements.
        if number {
            println!("number was three");
        }

        /*
        $ cargo run
        Compiling branches v0.1.0 (file:///projects/branches)
        error[E0308]: mismatched types
        --> src/main.rs:4:8
        |
        4 |     if number {
        |        ^^^^^^ expected `bool`, found integer

        For more information about this error, try `rustc --explain E0308`.
        error: could not compile `branches` due to previous error
        */
}};

    {
        let number = 3;

        if number != 0 {
            println!("number was something other than zero");
        }
        // This will print 'number was something other than zero'.
    }

    /// Rust includes else if for chaining conditions
    /// however match expressions are usually more readable.

    {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        // $ cargo run
        //     Compiling branches v0.1.0 (file:///projects/branches)
        //      Finished dev [unoptimized + debuginfo] target(s) in 0.31s
        //       Running `target/debug/branches`
        // number is divisible by 3
    }

    /// in rust if is an expression that returns a value
    {
        let condition = true;
        let number = if condition { 5 } else { 6 };
            // this conditionally assigns 5 or 6 to number.

        println!("The value of number is: {number}");

        // $ cargo run
        //     Compiling branches v0.1.0 (file:///projects/branches)
        //      Finished dev [unoptimized + debuginfo] target(s) in 0.30s
        //       Running `target/debug/branches`
        // The value of number is: 5
    }

    ignore!{
    {   // this is an example of mismatched types in an if else expression
        let condition = true;

        let number = if condition { 5 } else { "six" };

        println!("The value of number is: {number}");

        // $ cargo run
        //    Compiling branches v0.1.0 (file:///projects/branches)
        // error[E0308]: `if` and `else` have incompatible types
        //  --> src/main.rs:4:44
        //   |
        // 4 |     let number = if condition { 5 } else { "six" };
        //   |                                 -          ^^^^^ expected integer, found `&str`
        //   |                                 |
        //   |                                 expected because of this
        //
        // For more information about this error, try `rustc --explain E0308`.
        // error: could not compile `branches` due to previous error
    }};


    {

    }

    {

    }

}
