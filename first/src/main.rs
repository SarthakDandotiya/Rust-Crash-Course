fn main() {
    // This is a Comment

    // IMPPORTANT: Every Statement in Rust should terminate with a 'Semicolon (;)'

    // Rust puts a lot of emphasis on immutability. Immutability while keeping Concurrency in mind.

    // We use the 'let' keyword to declare variables in Rust which is very similar to what Javascript does now, making things pretty easy.

    // Rust mainly has 4 main "Scalar" Types:
    // Integer, Floating Point, Boolean and Characters
    // Then there are different types of each, like Integer could be signed & 8-bit ie.Specific to save space and in Rust Memory-Management is also a key factor.

    
    let x = 1; // Integer
    let y = 2.0; // Floating Point Number
    let me = "Sarthak Dandotiya"; // Character/String Type
    let b = true; // Boolean [Can only be Either True or False]

    let dynamic_math = 8 * 8;
    let dynamic_math_2 = 8 - 2 + 8 * 2; // General Preceedence of operators is followed as other programming languages

    // It is recommended or wished that you use 'Snake Case' in Rust and you will be prompted every now and then to do so.

    // Rust will also complain if you try to run or compile a program with un-used variables.

    // Some other data-types
    let my_array = [1, 2, 3, 4, 5, 6, 7]; // Array, similar to other programming languages

    let my_tuple = (5, 5.0, "Sarthak"); // Tuple, very much like in Python

    let (dynamic_x, dynamic_y, dynamic_z) = my_tuple; // Using a tuple to create variables on the go.

    // These are the basic Types. There are more, its suggested to go through them once but the ones mentioned here were the simplest ones to know.
    
    println!("x = {}", x);
    println!("y = {}", y);
    println!("me = {}", me);
    println!("b = {}", b);
    println!("dynamic_math = {}", dynamic_math);
    println!("dynamic_math_2 = {}", dynamic_math_2);
    println!("Value at index 5 in my_array = {}", my_array[5] ); // Indexing starts from 0
}
