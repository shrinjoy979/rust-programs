fn main() {
    let a: i8 = 10;
    let b: i8 = 20;
    let sum: i8 = do_sum(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
}

fn do_sum(a: i8, b: i8) -> i8 {
	return a + b;
}

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
