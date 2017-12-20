fn main(){

    // let test;
    // The above doesn't work in Rust, a variable must be given a type upon
    // declaration otherwise Rust cannot infer the type and will not create
    // the variable

    let x = 5;
    println!("x is {}", x);
    // x = 6;
    // println!("x is {}", x);

/*
    Given the immutability of variables in Rust, the first assignment
    is fine and can be used throughout the program, except when the
    programmer attempts to reassign the value of x. Much like strings
    in Java, these variables cannot be changed directly, but must be
    altered using methods and storing the result elsewhere (or the same
    variable, I think)
*/

    let mut y = 7;
    println!("y is {}", y);
    y = 10;
    println!("y is now {}", y);

    /*
        If when we declare a variable we add the mut keyword, this indicates
        the variable is not to be treated as immutable. This allows us to directly
        alter the contents of a variable.

        To my surprise, Rust still has constants. I suppose this would be useful
        for instances when you are absolutely sure you're never going to change
        the contents of a variable, or you want to eliminate "magic numbers".
    */

    const NON_MAGIC_NUMBER: u32 = 100_000;
    println!("constant is {}", NON_MAGIC_NUMBER);

    /*
        The Rust book uses the underscore between the zeros above for some reason.
        I suspect is may be a readability choice as the println produces the
        same output with and without the underscore. (It is for readability,
        doesn't affect value stored in variable)
    */

    let z = 6;
    let z = z + 1;
    let z = z * 3;
    println!("z is {}", z);

    /*
        The Rust book describes shadowing which seems to be a way to sidestep
        the immutability of variables. The book claims this is useful for when
        you need to alter the value bound to a variable which is then persistent
        through the rest of the program without using the mut keyword. The book
        describes this as reassigning a variable. Makes sense, but I feel like
        it allows the programer to ignore the immutability and potentially get
        into trouble.
    */

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
    /*
        So unlike Java and C, we don't have to explicitly state the variable's type
        at declare time. Whatever we assign the variable to be determines its type,
        similar to Python. I'm not sure if Python allows two variables with the same
        name, but differing types. (It does)
    */

    /*
        ***********************************************************
        ***Below are the scalar-type variables available in Rust***
        ***********************************************************
    */
    let a: i8 = 0x00; // Signed 8-bit integer, zero as hex
    let b: u8 = b'Q'; // Unsigned 8-bit integer, Q as byte (Only possible for u8)
    let c: i16 = 0o00; // Signed 16-bit integer, zero as octal
    let d: u16 = 0b0000; // Unsigned 16-bit integer, zero as binary
    let e: i32 = 0; // Signed 32-bit integer
    let f: u32 = 0; //Unsigned 32-bit integer
    let g: i64 = 0; // Signed 64-bit integer
    let h: u64 = 0; // Unsigned 64-bit integer
    let i: isize = 0; // Signed 32 or 64-bit integer, depends on your architecture
    let j: usize = 0; // Unsigned 32 or 64-bit integer, depends on your architecture

    println!("a: {}\nb: {}\nc: {}\nd: {}\ne: {}\nf: {}\ng: {}\nh: {}\ni: {}\nj: {}", a, b, c, d, e, f, g, h, i, j);

    let big_float = 3.14; // Floats default to 64-bit size unless specified
    let small_float: f32 = 2.79; // Can specify a 32-bit float
    println!("big float: {}\nsmall float: {}", big_float, small_float);

    let t = false;
    let q: bool = true; // Boolean declaration can either be implicit or explicit

    println!("t: {}\nq: {}", t, q);


    let chr = 'a';
    let another_chr = 'B';

    println!("chr: {}\nanother_chr: {}", chr, another_chr);

    /*
        Rust also has a char type like C, but Rust can handle unicode size
        characters. No sizeof in Rust yet, but checking online reveals that
        a unicode character depends on the type of unicode used, I would
        guess somewhere its something between UTF-16 and UTF-32 given the
        ranges available for use as described in the book (U+0000 - U+D7FF,
        & U+E000 - U+10FFFF)
    */

    /*
        ****************************************
        ***Compound types are described below***
        ****************************************
    */

    let tup: (i32, u16, u8) = (2_000_000, 32, b'G');
    println!("first: {}\nsecond: {}\nthird: {}", tup.0, tup.1, tup.2);


    /*
        A tuple can hold multiple primitive variable types, treated as one
        compound type. Access each value in a tuple using (NAME).(NUMBER OF ELEMENT)
        Can also access a specific element in a tuple by assigning it to a variable,
        or by creating another tuple with the same number of elements, but it
        consists of variables
    */

    let tup_zero = tup.0;
    println!("tup_zero: {}", tup_zero);

    let (tup_one, tup_two, tup_three) = tup;
    println!("tup_one: {}\ntup_two: {}\ntup_three: {}", tup_one, tup_two, tup_three);

    let array = [1, 2, 3, 4, 5, 6, 7];
    println!("array[3]: {}", array[3]);

    /*
        Arrays behave similar to C or Java, but it seems like you can't
        declare an empty array and fill it later. If you need to fill an
        array later, the book suggests a vector, but that'll be covered later
        apparently. Unlike tuples, elements of an array must all be the same
        type, not unlike arrays in C or Java.
        Unlike C, Rust has bounds checking for arrays and will throw an
        index out of bounds runtime error, which is pretty nice!
    */


    another_func();
    /*
        We can declare other functions in our program (Using fn keyword), much
        like any other programming language. We need to have a main function as our
        starting point. Rust also uses snake_case for function and variable
        names. In Rust, where a function is defined doesn't matter. It can be
        defined before or after its usage, unlike earlier versions of C.
    */

    print_value(360);
    multiple_values(3, 4);

    /*
        ************************************
        **** Statements and Expressions ****
        ************************************
        A statement is an instruction which performs some action and
        does not return a value. Expressions performs some instruction,
        but do return a value. Rust differs from C here in that variable
        assignment is not treated as an expression. So let x = 6 would return
        6 in C, but returns nothing in Rust. However, the example below
        does return a value even though a statement is part of the
        expression.

        It is important to note that expressions and statements
        differ in semicolon usage. A statement requires a semicolon
        while an expression requires no semicolon. I don't mean it is
        suggested to not place a semicolon after an expression, rather
        that you strictly should not place a semicolon after an expression
        otherwise you'll excounter a runtime error
    */

    // Here, exp will contain the value 15
    let hold = 1;
    let exp = {
        let hold = 7;
        hold + 8 // Note: no semicolon since this is an expression
    };

    println!("exp is: {}", exp);

    let exp = mult(exp);

    println!("exp is now {}", exp);


    /*
        ********************
        ****Control Flow****
        ********************
        if-else statements behave as expected.
        Unlike C, the comparison in if statments must consist of booleans,
        we cannot use integers as stand-ins for booleans. We can use
        multiple if else-if statements in a block to further control
        program execution.

        A nifty feature in Rust is using if to control variable
        assignment. See the example below.
        In Rust, if is treated as an expression, so we can use it to return
        some value based on a condition. These values must all be the same type
        though. We can't return an integer or string based on a condition, must
        be one or the other.
    */

    if hold > 20 {
        println!("hold was greater than 20");
    } else {
        println!("hold was less than 20");
    }

    let result = if t {
        42
    } else {
        76
    };

    println!("result is: {}", result);


    /*
        ***************
        *****Loops*****
        ***************
        The keyword loop will create an infinite loop that cannot be escaped
        unless a break is added.

        while behaves as expected in other languages we can use true as the
        condition, but Rust suggests you use loop in that case since you're
        creating an infinite loop already. We can also pass expressions to
        be treated as conditionals to control how many times the while loop loops.

        For loops differ a little from Java or C. Rather than the typical
        for loop with an initialization, the for loops in Rust are similar to
        those in Python. If we want to loop over the elements in an array,
        we seem to get an iterator for the array and use that to loop rather
        than manually accessing each element by index. This helps prevent
        index out of bounds errors that could arise by error or deleting an
        element from the array (technically vector). We can also iterate in
        reverse using .rev()
        If we want specific elements, we can also specify the bounds explicitly.
        In that case, the first element is inclusive and the last element
        is exclusive.
    */

    loop {
        println!("Do forever unless broken");
        break;
    }

    let mut counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter = counter - 1;
    }

    for element in array.iter() {
        println!("value: {}", element);
    }

    for element in (1..5).rev() {
        println!("rev value: {}", element);
    }

}

fn another_func(){
    println!("Hello from another function!");
}

/*
    For functions with parameters, you MUST declare the type of the
    paramter to be passed. I suspect this would help with eliminating
    null pointers being passed, but I'm not positive yet.
*/
fn print_value(x: i32){
    println!("from print_value, x is: {}", x);
}

fn multiple_values(x: u32, y: i8){
    println!("from multiple_values, x: {}, y: {}", x, y);
}


/*
    To declare a return type for a function, use -> followed by the
    type to be returned.
*/
fn mult(x: i32) -> i32{
    x * 2
}
