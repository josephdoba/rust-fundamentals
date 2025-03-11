fn main() {
    println!("Struct Tutorial");

    /*
    a struct (aka structure) is a custom data type, that lets you bundle
    and name multiple related values that make up a group. in OOP, struct is like
    an objects data attributes.

    We'll compare tuples and structs later on
     */

    //struct syntax:
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // How to instantiate an object
    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("example@example.com"),
        sign_in_count: 3,
    };

    // dot notation works!
    println!("{}", user1.username);

    //take note of the arrow here, its called Field Init Shorthanding -- since its redundant to write the arguements twice, doing this just assigns those args as the value into that key/value pair.
    fn build_user(email: String, username: String) -> User { 
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2 = build_user("example@example.com".to_string(), "bob".to_string());

    println!("{:?}", user2); // I don't understand yet why this needs to be in debug mode before I can print out the object, and printed with the :? flag.... ooh I see, its part of Rusts design philosphy of being explicit. Got it! 


    struct Rectangle {
        width: u32,
        height: u32
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("{}",area(&Rectangle { width: (30), height: (25) }));
    println!("{}", area(&Rectangle{ width: (10), height: (12) }));



}