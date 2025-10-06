fn main() {
    //You can instantiate a new struct, this function creates it using arguments
    let mut user1 = create_user(true, "Keith", "keith@myemail.com");
    println!("{}, {}, {}", user1.active, user1.username, user1.email);

    //Because we declared it as mutable, we can edit with dot notation
    user1.active = false;
    user1.username = String::from("Bobby");

    println!("{}, {}, {}", user1.active, user1.username, user1.email);

    //You can use struct update to create instances from other instances
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("bob@bobworld.com"),
    };

    println!("{}, {}, {}", user2.active, user2.username, user2.email);

    //You can inherit everything you don't specify using the .. syntax
    let user3 = User {
        email: String::from("newcoolguy@coolguys.com"),
        ..user2
    };

    println!("{}, {}, {}", user3.active, user3.username, user3.email);

    //Rust also has something called tuple structs
    //There is no names associated with fields, just types

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);
}

//You can create structs which are custom data types (like in OOP)
//They operate using key value pairs
//You can access them using dot notation and they can be mutable
//When you create an instance of one, you can define the keys in any order

struct User {
    active: bool,
    username: String,
    email: String,
}

fn create_user(active: bool, username: &str, email: &str) -> User {
    //Since we are returning a struct from this function, we don't need to use let
    User {
        active, //If the name is the same as the argument, you can just use shorthand
        username: String::from(username),
        email: String::from(email),
    }
}
