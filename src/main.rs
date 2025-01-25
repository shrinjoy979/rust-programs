// Ownership

/*
fn main() {
    let x = 1; // crated on stack
    let y = 3; // created on stack

    println!("{}", sum(x, y));
    println!("Hello, world!");
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}
*/

/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // Compiles now
}
*/

fn main() {
    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}

// Borrowing and references
