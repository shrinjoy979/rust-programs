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