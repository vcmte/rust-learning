#[derive(Debug)]
#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn demonstration() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    // ################################################

    v.extend([4, 5]);
    v.push(6);

    // ################################################
 
    indexing(&v);
    println!();

    borrowing(&mut v);
    println!();

    iterating(&v);
    println!();

    vectoring_enum();
    println!();

    slicing(&v);
    println!();

    changing(&mut v);
    println!();
    
    capacity();
    println!();

    paraphernalia(&mut v);
    println!();
}

fn indexing(v: &[i32]) {
    let third = &v[2];
    println!("The third element is {third}\n");

    // ################################################

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}\n"),
        None => println!("There is no third element.\n"),
    }

    // ################################################

    let does_not_exist = v.get(100);
    println!("{does_not_exist:?}\n"); // None

    // let does_not_exist = &v[100];
    // println!("{does_not_exist}"); // Panic!!

    // ################################################

    // v.first() -> Some(1), v.last() -> Some(5)
    println!("v: from {:?} to {:?}\n", v.first(), v.last());
}

fn borrowing(v: &mut Vec<i32>) {
    let first = &v[0];
    println!("The first element is: {first}\n");
    v.push(6);

    // Vector `v` is already borrowed as mutable, so the code below will cause an error
    // println!("The first element is: {first}");
}

fn iterating(v: &[i32]) {
    for i in v {
        println!("{i}");
    }
    println!();

    let mut tmp = vec![100, 32, 57];
    for i in &mut tmp {
        *i += 50;
    }
    println!("{tmp:?}\n");
}

fn vectoring_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}\n");
} // <- row goes out of scope and is freed here

fn slicing(v: &[i32]) {
    // let u: &[_] = v;
    let u = &v[0..2];

    println!("{u:?}\n");
}

fn changing(v: &mut Vec<i32>) {
    let idx = 10;
    let idx = idx.min(v.len()) + 1;

    v.insert(0, 0);
    v.insert(idx, 10);
    println!("{v:?}\n");

    // ################################################

    let last = v.pop().unwrap_or(-1);
    println!("{last:?}");
    println!("{v:?}\n");

    v.remove(0);
    println!("{v:?}\n");

    v.swap_remove(2);
    println!("{v:?}\n");

    v.clear();
    println!("{v:?}\n");

    // ################################################

    *v = vec![5, 1, 2, 3, 4, 5];
    println!("{v:?}\n");
}

fn capacity() {
    let mut v = Vec::with_capacity(100);

    for i in 0..50 {
        v.push(i);
    }
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());

    v.resize(100, 0);
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());

    v.reserve_exact(1);
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());

    v.extend([0, 0]); // second element is causing reallocation
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());

    v.clear();
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("Len: {}, Cap: {}\n", v.len(), v.capacity());
}

fn paraphernalia(v: &mut Vec<i32>) {
    println!("Vector `v`: {v:?}");
    println!("Length of vector `v` is {}", v.len());
    println!("Is vector `v` empty? {}", v.is_empty());
    println!("Does vector `v` contain 5? {}", v.contains(&5));
    println!("Splitted vector `v` at index 2: {:?}", v.split_at(2));

    v.reverse();
    println!("Reversed vector `v`: {v:?}"); // After reverse above

    v.resize(10, 0);
    println!("Resized vector `v` with additional zeros: {v:?}");

    let mut truncated = v.clone();
    truncated.truncate(3);
    println!("Truncated vector `v` for 3 elements: {truncated:?}");

    v.swap(3, 4);
    println!("Vector `v` after swapment: {v:?}");

    v.extend(truncated);
    println!("Vector `v` after appendment of vector `truncated`: {v:?}");

    v.sort_unstable(); //v.sort();
    println!("Sorted vector `v`: {v:?}");

    v.dedup();
    println!("Deduplicated vector `v`: {v:?}");

    v.fill(0);
    println!("Filled with zeros vector `v`: {v:?}");
}
