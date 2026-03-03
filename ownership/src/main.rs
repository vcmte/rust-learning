fn main() {
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let mut x = 5;
    let y = x;
    x = 6;
    //y = 7; <-------------------------|
    println!("{x}, {y}"); // 6, 5 <----|

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");

    let new = String::from("hello");
    takes_ownership(new);
    // println!("{new}"); // error, borrow of moved value

    let z = 5;
    makes_copy(z);

    let st1 = gives_ownership(); // gives_ownership moves its return value into st1
    println!("{st1}");

    let st2 = String::from("hello"); // st2 comes into scope

    // st2 is MOVED into takes_and_gives_back, which also MOVES its return value into s3
    let st3 = takes_and_gives_back(st2);
    println!("{st3}");

    let example = String::from("hello");
    let (s2, len) = calculate_length(example);
    println!("The length of '{s2}' is {len}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and MOVES out to the calling function
}

// This function take a String and returns a String
// a_string comes into scope
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and MOVES out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
