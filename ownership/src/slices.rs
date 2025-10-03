pub fn slices() {
    //String slices
    //String slices is a reference to a contiguous sequence of elements in a string
    //It uses [start_index..end_index]

    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");
}
