const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const SPECIAL_FIRST_GIFT: &str = "A partridge in a pear tree";

fn main() {
    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            DAYS[day]
        );

        if day == 0 {
            println!("{SPECIAL_FIRST_GIFT}\n");
            continue;
        }

        let gifts_today = &GIFTS[1..=day];
        for gift in gifts_today.iter().rev() {
            println!("{gift},");
        }
        println!("{}.\n", GIFTS[0]);
    }
}
