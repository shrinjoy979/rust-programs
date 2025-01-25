fn main() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    println!("Before update: {}", s);

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    // pointer change, after capacity bacome very big
    for _ in 0..100 {
        s.push_str(" and some additional text");
        println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    }
}

/*
fn main() {
    let a: i8 = 10;
    let b: i8 = 20;
    let sum: i8 = do_sum(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
}

fn do_sum(a: i8, b: i8) -> i8 {
	return a + b;
}
*/

// pub fn: A function marked as public, allowing it to be called from other modules or crates.
/*
    pub fn main() {
        let str = String::from("Shrinjoy Saha");
        println!("First name {}", get_first_name(str))
    }

    pub fn get_first_name(str: String) -> String {
        let mut first_name = String::from("");
        for c in str.chars() {
            if c == ' ' {
                break
            }
            first_name.push(c);
        }
        return first_name;
    }
*/

// fn main() {
    // println!("Hello, world!");

    /* 
        let x: i8 = -5;
        let y: u32 = 1000;
        let z: f32 = 1000.01;

        print!("x: {}, y: {}, z: {}", x, y, z);
    */

    /*
        let mut x: i32 = 8;

        for i in 0..1000 {
            x = x + 100;
        }

        print!("x = {}", x);
    */

    /*
        let is_male: bool = false;
        let is_above_18: bool = true;

        // let mut is_above_18: bool = true;
        // is_above_18 = false;
        
        if is_male {
            println!("You are a male");

        } else {
            println!("You are not a male");
        }

        if is_male && is_above_18 {
            print!("You are a legal male");
        }
    */

    /*
        let greeting = String::from("hello world");
        println!("{}", greeting);
    */

    // if we want to print 1000th index, of the string, it will give error
    // beacuse rust compilar, is not sure that 1000th index have string or not
    // print!("{}", greeting.chars().nth(1000))

    // Option<char> means you have a variable that could either contain a value of type OR represent the absence of a value

    /*
        let char1 = greeting.chars().nth(1000);

        match char1 {
            Some(c) => println!("{}", c),
            None => println!("No character at index 1000"),
        }
    */

    /*
        let is_male = false;
        let is_above_18 = true;
        
        if is_male {
            println!("You are a male");

        } else {
            println!("You are not a male");
        }

        if is_male && is_above_18 {
            print!("You are a legal male");
        }
    */ 
// }
