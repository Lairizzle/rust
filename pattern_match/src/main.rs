//match is a control flow construct
//It allows you to compare a value against a series of patterns
//match is like a coin sorting machine. Coins slide down a track with different sized holes
//and each coin falls through the first hole that it fits into

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//You can also match a bind to parts of the values that match the pattern
//You can extract values our of enum variants
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    //If we write a function that takes Option<i32>
    //If we want to get a value, if it exists and then add one to it
    //If there is no value it should return None
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    check_dice_roll();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny and it worth 1 cent");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //If we add the state value we can match to it
        Coin::Quarter(state) => {
            println!("This is a quarter worth 25 cents and it is a state quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        //The passed value Some(5) matches Some(i)
        //So it executes the code
        Some(i) => Some(i + 1),
    }
}

//There is also a catch all pattern with the _ placeholder
fn add_fancy_hat() {}
fn remove_fancy_hate() {}
fn reroll() {
    println!("You didn't roll a 3 or 7 so you should reroll the dice.");
}

fn check_dice_roll() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hate(),
        // _ is a special pattern that matches any value and does not bind to it
        _ => reroll(),
    }
}
