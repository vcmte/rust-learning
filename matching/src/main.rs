#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // --snip-- //
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let wallet = [
        Coin::Penny,
        Coin::Penny,
        Coin::Penny,
        Coin::Quarter(State::Alabama),
        Coin::Quarter(State::Alaska),
        Coin::Dime,
        Coin::Dime,
        Coin::Nickel,
    ];

    let mut balance: u32 = 0;
    for coin in &wallet {
        balance += in_cents(coin);
    }

    println!("Your balance is: {balance}");

    // #######################################

    let five = Some(5);
    let six = unstable_increment(five);
    let none = unstable_increment(None);

    println!("Results of unstable incrementation: {five:?}, {six:?}, {none:?}");

    // #######################################

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // other => move_player(other),
    }

    // #######################################

    let config_max = Some(3u8);

    #[allow(clippy::single_match)]
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("Anything else!");
    }

}

fn add_fancy_hat() {
    println!("Player has got a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Player has lost a fancy hat :(");
}

fn reroll() {
    println!("Dice was rerolled!");
}

/*
fn move_player(num_spaces: u8) {
    println!("Player was moved forward by {num_spaces} spaces");
}
*/

#[allow(clippy::manual_map)]
fn unstable_increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None, // remove this and that will cause an error, cuz match should handle all possible cases
    }
}

fn in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny was in your wallet!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {state:?} was in your wallet!");
            25
        }
    }
}
