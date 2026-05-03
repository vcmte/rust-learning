use std::collections::HashMap;

pub fn map_demonstration() {
    scoreboard();
    println!();

    owner_movement();
    println!();

    overwriting();
    println!();

    secure_inserting();
    println!();

    recreating();
    println!();

    quick_construction();
    println!();

    capacity_behavior();
    println!();

    presence_and_lookup();
    println!();

    mutable_access();
    println!();

    removing();
    println!();
}

pub fn set_demonstration() {
    println!();
}

fn quick_construction() {
    let array: [(String, (u8, u8, u8)); 2] = [
        (String::from("black"), (0, 0, 0)),
        (String::from("white"), (255, 255, 255)),
    ];
    let vector: Vec<(String, (u8, u8, u8))> = vec![
        (String::from("black"), (0, 0, 0)),
        (String::from("white"), (255, 255, 255)),
    ];

    let map_from_array = HashMap::from(array);
    let map_from_vector: HashMap<String, (u8, u8, u8)> = vector.into_iter().collect();

    println!("map_from_array: {map_from_array:?}, map_from_vector: {map_from_vector:?}");
    dbg!(assert_eq!(map_from_array, map_from_vector)); // success
}

fn capacity_behavior() {
    let mut map = HashMap::with_capacity(32);
    map.insert(0, 56);
    // Output: map len: 1, map cap: 56 (!!!)
    // Why 56 instead of 32?
    // - `with_capacity(32)` guarantees space for AT LEAST 32 elements.
    // - Buckets count must be a power of 2.
    // - Max load factor is 7/8.
    // - 32 items / (7/8) = 36.5 buckets needed -> rounds up to 64.
    // - 64 buckets * 7/8 = 56 actual capacity.
    println!("map len: {}, map cap: {}", map.len(), map.capacity());

    for i in 1..56 {
        map.insert(i, 56 - i);
    }
    println!("map len: {}, map cap: {}", map.len(), map.capacity()); // Everything is fine!

    // We're on limit of capacity, so let's reserve some memory for one more element
    // Haha, ofc we'll get space for 56 more freaking elements!
    map.reserve(1);
    println!("map len: {}, map cap: {}", map.len(), map.capacity());
    // Let's fill all the new allocated space
    for i in 57..113 {
        map.insert(i, 113 - i);
    }
    println!("map len: {}, map cap: {}", map.len(), map.capacity()); // Everything is fine!

    // Adding new element over capacity limit
    map.insert(113, -1); // REALLOCATION! cap doubles: 112 -> 224
    println!("map len: {}, map cap: {}", map.len(), map.capacity()); // now we got 112 more slots for our elements

    // Clearing & shrinking
    map.clear();
    println!("map len: {}, map cap: {}", map.len(), map.capacity()); // map len: 0, map cap: 224
    map.shrink_to_fit();
    println!("map len: {}, map cap: {}", map.len(), map.capacity()); // map len: 0, map cap: 0
}

fn presence_and_lookup() {
    let map = HashMap::from([
        (String::from("Blue"), 20),
        (String::from("Green"), 40),
        (String::from("Yellow"), 70),
        (String::from("Red"), 30),
        (String::from("Purple"), 50),
    ]);

    println!(
        "Is blue team present?: {}, is pink team present?: {}",
        map.contains_key("Blue"),
        map.contains_key("Pink")
    );

    if let Some((team, score)) = map.get_key_value("Blue") {
        println!("Team {team}: {score}");
    }

    println!("Team Red: {}", map["Red"]); // make sure key exist in map
    // println!("Team Black: {}", map["Black"]); // this code will panic, because there is no black team present
}

fn mutable_access() {
    let mut time: HashMap<String, u8> = HashMap::from([
        (String::from("hours"), 0),
        (String::from("minutes"), 0),
        (String::from("seconds"), 0),
    ]);

    if let Some(seconds) = time.get_mut("seconds") {
        for _ in 0..60 {
            *seconds += 1;
        }
    }

    // Why not just two get_mut calls? Because borrow checker forbids
    // two mutable borrows of the same map at once. get_disjoint_mut
    // bypasses this by taking all keys upfront and checking they're distinct.
    if let [Some(minutes), Some(seconds)] = time.get_disjoint_mut(["minutes", "seconds"])
        && *seconds == 60
    {
        *minutes += 1;
        *seconds = 0;
    }

    // and_modify alone does nothing if key is absent
    time.entry(String::from("hours"))
        .and_modify(|hours| *hours = 23);

    println!(
        "Current time: {:02}:{:02}:{:02}",
        time["hours"], time["minutes"], time["seconds"]
    );
}

fn removing() {
    let mut map = HashMap::from([
        (String::from("John"), 25),
        (String::from("Jack"), 3),
        (String::from("Joseph"), 50),
        (String::from("Jake"), 35),
        (String::from("Bob"), 42),
        (String::from("Tony"), 22),
    ]);
    println!(
        "Winner had {:?} points!",
        map.remove("Joseph").unwrap_or_default()
    );
    println!("Winner have {:?} points now!", map.remove("Joseph"));
    if let Some((name, score)) = map.remove_entry("Jack") {
        println!("Loser {name} had {score} points!");
    }

    let mut from_map: Vec<(String, (i32, bool))> = Vec::with_capacity(map.len());
    for (k, v) in map.drain() {
        from_map.push((k, (v + 3, v + 3 >= 30)));
    }
    println!("map len: {}, map: cap: {}", map.len(), map.capacity());

    let mut map: HashMap<String, (i32, bool)> = from_map.into_iter().collect();
    println!("map len: {}, map: cap: {}", map.len(), map.capacity());

    map.clear();
    println!("map len: {}, map: cap: {}", map.len(), map.capacity());

    map.shrink_to_fit();
    println!("map len: {}, map: cap: {}", map.len(), map.capacity());
}

fn scoreboard() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    // Memory effective way
    let score = scores.get(&team_name).unwrap_or(&0);
    println!("{team_name} has {score} score!");

    // Standalone copy way
    let score = scores.get(&team_name).copied().unwrap_or_default();
    println!("{team_name} has {score} score!");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn owner_movement() {
    let field_name = String::from("Favorite volor");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{map:?}");

    // println!("{field_name}: {field_value}"); // error: borrow of moved value
}

fn overwriting() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

fn secure_inserting() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn recreating() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
