// Functions

// Functions are declared using the 'fn' keyword.

fn main() {
    hello_world(); // Function Call
    print_name("Sarthak Dandotiya"); // Passing Arguments
    add(5, 6);
}

// Naming convention for functions is tha same as variables, we can't have somethin like numbers at the start or keywords as function names
fn hello_world() {
    println!("Hello World [From Function]!");
}

// Rust will throw an error if we do not put & in front of str as is will not know the size of the argument at compile time
// Adding '&' is a work around for that
fn print_name(name: &str) {
    println!("Your name is: {}", name);
}

// Passing in much specific values here ie.8-bit intergers
fn add(x :i8, y :i8) {
    println!("{}",x + y);
}