//Structs gave us a way to group together a key and value
//Enums give you a way to say a value is one of a possible set

//One example of a enum could be IP addresses
//There are two versions of IP addresses, IP4 and IP6
//We can enumerate all possible variants

#[derive(Debug)]
enum IPAddrKind {
    //V4,
    //V6,
    V4(String), //By adding (String) we create a function that takes a string
    V6(String), //IpAddrKind::V6()
} //This is now a custom data type

fn main() {
    //We can create instances of these variants
    //let four = IPAddrKind::V4;
    //let six = IPAddrKind::V6;
    //The variants are namespaced under the identifier (IPAddrKind)

    //You could use a struct and define the type there
    //Or you can put the data directly into the variant
    let four = IPAddrKind::V4(String::from("127.0.0.1"));
    let six = IPAddrKind::V6(String::from("::1"));
    route(four);
    route(six);
    println!("Hello, world!");
}

//We could define a function that takes any IPAddrKind
fn route(ip_kind: IPAddrKind) {
    println!("The type of IP is {ip_kind:?}");
}
