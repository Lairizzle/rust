//This is a good example of how you might use a struct in a program
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let scale = 2;
    let rectangle = Rectangle {
        //You can use dbg to value of a specific a field
        width: dbg!(30 * scale),
        length: 50,
    };

    println!("rectangle is {rectangle:?}");
    dbg!(&rectangle);

    println!(
        "The area of the rectangle is {}",
        calculate_area(&rectangle)
    );
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}
