fn main() {
    //Instead of moving, you can use references
    let s1 = String::from("Hello");
    //The & in front of string represent a reference
    //This allows you to refer to a value without taking ownership
    //A reference is like a pointer, but it is guaranteed to point to a valid value
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} //The s goes out of scope here, but it doesnt own what it references
//This means the string is not dropped
//This is called borrowing
//You cannot change what you borrow, they are immutable by default
