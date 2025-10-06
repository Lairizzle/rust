#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//You can define the function within the context of the struct
//Everything inside the impl block will be associated with the struct
//This is using the method syntax to call the area function on the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //You can have multiple functions that use the method syntax
    //They can also take multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //All functions defined within an impl block are associated functions
    //If you define a function that doesn't have self as their first parameter
    //They are not a method because they dont need an instance of the type to work with
    //This is like the String::from function

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    //To call this function we have to use the :: syntax
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "Can the one rectangle hold the other? The answer is {}",
        rect2.can_hold(&rect3)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        //Now we can call the function on the struct using dot notation
        rect1.area()
    );

    //Calling the associated NON METHOD function
    let sq = Rectangle::square(3);
    println!("{sq:?}");
}
