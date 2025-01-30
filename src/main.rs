/*
fn main() {
    let mut vec = Vec::new() ;
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println! ("{:?}", vec); // Vec is a struct, that's why we use {:?} to print data
}
*/

/*
fn main() {
    let mut vec = Vec::new() ;
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println! ("{:?}", even_filter(vec)); // Vec is a struct, that's why we use {:?} to print data
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut newVac = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            newVac.push(val);
        }
    }

    return newVac;
}
*/

/*
fn main() {
    let mut vec = Vec::new() ;
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println! ("{:?}", even_filter(&vec)); // Evenly filtered array

    println!("{:?}",vec); // Original array
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut newVac = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            newVac.push(*val); // De-reference
        }
    }

    return newVac;
}
*/

// Hashmap
/*
use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("Shrinjoy"), 27);
    users.insert(String::from("Saha"), 26);

    // {
    //     "Shrinjoy": 27,
    //     "Saha": 26
    // }

    let first_user_age  = users.get("Shrinjoy"); // will return an option

    match first_user_age {
        Some(age) => println!("Age is {}", age),
        None => println!("User not found in Database")
    }
}
*/

/*
use std::collections::HashMap;

fn group_values_by_keys (vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert (key, value);
    }
    return hm;
}

fn main() {
    let input_vec = vec![(String:: from("harkirat"), 22), (String:: from("raman"), 32)];
    let hm = group_values_by_keys (input_vec);
    println!("{:?}", hm);
}
*/

/*
#iteration using for loop
fn main() {
    let va = vec![1, 2, 3];

    for val in va {
        print!("{}", val)
    }
}
*/

/*
//#Iterating after creating an `iterator
fn main() {
    let va = vec![1,2,3];
    let va_iter = va.iter();

    for val in va_iter {
        println!("{}", val);
    }
}
*/

/*
// #IterMut
fn main() {
    let mut va = vec![1, 2, 3];
    let iter = va.iter_mut();

    for  value in iter {
        *value = *value + 1;
    }

    println!("{:?}", va);
}
*/
/*
// Iterating using `.next`
fn main() {
    let nums = vec![1, 2, 3];
    let mut iter = nums.iter();

    // iter.next return's option
    while let Some(val) = iter.next() {
        print!("{}", val);
    }
}
*/

/*
//#IntoIter
fn main() {
    let nums = vec![1, 2, 3];
    let iter = nums.into_iter();

    // iter.next return's option
    for  value in iter {
        println!("{}", value)
    }
}
*/

/*
fn main() {
    let v1 = vec![1,2,3];
    let iter = v1.iter();

    // let iter2 = iter.map(|x| x+1);
    let iter2 = iter.filter(|x| *x % 2 == 0);

    for value in iter2 {
        print!("{}", value);
    }
}
*/

/* 
// Basic String Operation
fn main() {
    let mut name = String::from("Shrinjoy"); // Create String

    name.push_str(" Saha"); // Mutating String

    name.replace_range(8..name.len(), ""); // Deleting from a String
    println!("name is {}", name);
}
*/

/*
// Write a function that takes a string as an input And returns the first word from it
fn main() {
    let givenStr = String::from("Shrinjoy Saha");
    let ans = get_first_word(givenStr);
    println!("{}", ans);
}

fn get_first_word(str: String) -> String{
    let mut ans = String::from("");

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }

    return ans;
}

The above solution have problem
We take up double the memory
If the `name` string gets `cleared` , `ans` still has `hello` as the value in it
*/

/*
// more optimal way using slice
fn main() {
    let mut givenStr = String::from("Shrinjoy Saha");
    let ans = get_first_word(&givenStr);

    println!("{}", ans);
}

fn get_first_word(str: &String) -> &str{
    let mut index = 0;

    for i in str.chars() {
        if i == ' ' {
            break;
        }

        index = index + 1
    }

    return &str[0..index];
}
*/

// trait
pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User name is {}, and age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Shrinjoy"),
        age: 27,
    };

    println!("{}", user.summarize());
}