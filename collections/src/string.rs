pub fn demonstration() {
    let mut s = String::new();

    // ################################################

    from_literal(&mut s);
    println!();

    from_bytes();
    println!();

    pushing(&mut s);
    println!();

    concatenating();
    println!();

    length();
    println!();

    indexing(&mut s);
    println!();

    iterating(&s);
    println!();

    s = representing(s);
    println!();

    paraphernalia(&mut s);
    println!();
}

fn from_literal(s: &mut String) {
    s.push_str("foo");
    println!("{s}");

    *s = "foo".to_string();
    println!("{s}");

    *s = String::from("foo");
    println!("{s}");
}

fn from_bytes() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap_or_default();

    println!("{sparkle_heart}");
}

fn pushing(s: &mut String) {
    let bar = "bar";
    s.push_str(bar);
    s.push('s');

    println!("bar is {bar} and foorbar is {s}!");
}

fn concatenating() {
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let s = hello + &world; // note hello has been moved here and can no longer be used

    println!("{s}");

    // ################################################

    let result = String::with_capacity(11);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let result = result + &tic + "-" + &tac + "-" + &toe;
    println!("{result}");

    let result = format!("{tic}-{tac}-{toe}");
    println!("{result}");
}

fn length() {
    let hello_es = String::from("Hola");
    let hello_ru = String::from("Здравствуйте");

    println!(
        "Length of spanish variant - '{hello_es}' (in bytes): {}",
        hello_es.len()
    );
    println!(
        "Length of russian variant - '{hello_ru}' (in bytes): {}",
        hello_ru.len()
    );
}

fn indexing(s: &mut String) {
    // println!("{}", s[0]); // raising error

    *s = String::from("Здравствуйте");
    // let answer = &hello[0]; // should this return 'З'? or '208', as first byte? this is ambiguous semantics!
    // let first = &hello[0..1]; // valid code, but cause panic

    println!("{}", &s[0..2]); // works, output: 'З'
    println!("{:?}", s.chars().nth(1)); // works, output: Some('д')
    println!("{:?}", s.get(4..6)); // works, output: Some("р")
}

fn iterating(s: &str) {
    for c in s.chars() {
        println!("{c}");
    }

    for c in s.char_indices() {
        println!("{c:?}");
    }

    for c in s.bytes() {
        println!("{c}");
    }
}

fn representing(s: String) -> String {
    let ptr = s.as_ptr();
    let len = s.len();
    let capacity = s.capacity();
    println!("ptr: {ptr:?}, len: {len:?}, capacity: {capacity:?}");

    // Deconstruct the String into parts
    let (ptr, len, capacity) = s.into_raw_parts();
    println!("ptr: {ptr:?}, len: {len:?}, capacity: {capacity:?}");

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    println!("{s}");

    // Deconstruct the String into Vec<u8> with char bytes
    let bytes = s.into_bytes();
    println!("{bytes:?}");

    // We can re-build a String out of Vec<u8> with char bytes
    let s = String::from_utf8(bytes).unwrap_or_default();
    println!("{s}");

    s
}

fn paraphernalia(s: &mut String) {
    s.clear();
    s.push_str("   Hello, world of worlds!\naboba   ");
    println!("{s}");

    println!("{}", s.trim());
    println!("{}", s.trim_start());
    println!("{}", s.trim_end());
    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.replace("world", "fella"));
    println!("{}", s.replacen("world", "fella", 1));
    println!("{}", s.repeat(3));

    println!("{:?}", s.strip_prefix("   Hello, "));
    println!("{:?}", s.strip_suffix(" of worlds!   "));
    println!("{:?}", s.trim().split_once(", ")); // search from start
    println!("{:?}", s.trim().rsplit_once(", ")); // search from end
    println!("{:?}", s.trim().split_at(6));
    println!("{:?}", s.split_whitespace().collect::<Vec<_>>());
    println!("{:?}", s.split_terminator(',').collect::<Vec<_>>());
    println!("{:?}", s.split(&['o', 'r']).collect::<Vec<_>>());
    println!("{:?}", s.rsplit(&['o', 'r']).collect::<Vec<_>>());
    println!("{:?}", s.splitn(2, 'o').collect::<Vec<_>>());
    println!("{:?}", s.rsplitn(2, 'o').collect::<Vec<_>>());
    println!("{:?}", s.lines().collect::<Vec<_>>());

    s.insert_str(9, " (cough)");
    println!("{s}");
    s.insert(16, '!');
    println!("{s}");
    let popped = s.pop().unwrap_or_default();
    println!("{s}, popped symbol: {popped}");
    s.remove(0);
    println!("{s}");
    s.truncate(7);
    println!("{s}");
    s.replace_range(0..2, "");
    println!("{s}");
    s.clear();
    println!("String: {}, len: {}, cap: {}", s, s.len(), s.capacity());

    s.push_str("Hello, world!");
    println!("{:?}", s.contains("world"));
    println!("{:?}", s.starts_with("Hell"));
    println!("{:?}", s.ends_with('!'));
    println!("{:?}", s.find("world"));
    println!("{:?}", s.rfind('o'));
    println!("{:?}", s.is_empty());
    println!("{:?}", s.is_char_boundary(1));
}
