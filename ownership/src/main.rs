fn main() {
    //What is ownership?
    //This is how Rust manages memory

    //Ownership Rules:
    //Each value in Rust has an owner
    //There can only be one owner a time
    //When the owner goes out of scope, the value is dropped.

    //In the string example it will demonstrate how move and copy works
    string_example();

    //Some types have known size at compile time and are stored entirely on the stack
    //In this case copies of the actual values are quick to create
    //In essence, certain types are cloned automatically
    integer_example();

    //Passing values to a function will move or copy, the same as assignment.
    //This string will move ownership, when this happens s can no longer be accessed
    let s = String::from("Moving");
    take_ownership(s); // s cannot be accessed after this

    //In this case, the integer is copied and can still be used after the function
    let x = 5;
    makes_copy(x);
    println!("{x} got copied, so it can still be called!");

    //Return values can also transfer ownership
    let s1 = gives_ownership(); //The function moves its return value into the s1 variable
    println!("{s1}");

    //Return values can take ownership and then transfer it back
    let s2 = String::from("Hot Potato");
    let s3 = take_and_give_back(s2);
    println!("{s3}");
}

fn string_example() {
    let s1 = String::from("Hello"); // The varivale s1 is in scope
    let s2 = s1; //s1 is no longer valid and is out of scope. s1 was moved to s2.
    //If we tried to use s1 after this it would provide an error
    println!("{s2} was moved, so it can no longer be called as s1, only as s2");

    //If we wanted to create an actual copy of the variable, we would use clone.
    //By using a clone() method we are making a concious decision to copy the data
    //This clone() method may be more expensive in terms of memory
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("{s3} and {s4} can both be used because s3 and s4 were cloned"); // Both varibles still exist in memory because they are cloned.
} // The variables are out of scope now and no longer valid, rust calls drop 

fn integer_example() {
    //In the example below, the integer is cloned automatically and is not moved.
    //Types that implement the copy trait include but are not limited to; integer, bool, floats,
    //chars, tuples (if they contain types that implement copy)
    let x = 5;
    let y = x;

    println!("{x} and {y} can both be used because integers are copied automatically");
}

fn take_ownership(owned: String) {
    println!("{owned} was moved and is being printed by this function.");
}

fn makes_copy(copied: i32) {
    println!("{copied} was copied and used by this function, but it was still used after.");
}

fn gives_ownership() -> String {
    //The return value of this function is moved into the variable that called it
    let some_string = String::from("yours");
    some_string
}

fn take_and_give_back(s2: String) -> String {
    //The argument is moved into the function, then it is returned to the variable that called it.
    s2
}
