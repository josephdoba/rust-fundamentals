fn main() {
    println!("Struct Tutorial");

    /*
    a struct (aka structure) is a custom data type, that lets you bundle
    and name multiple related values that make up a group. in OOP, struct is like
    an objects data attributes.

    We'll compare tuples and structs later on
     */

    //struct syntax:
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
    println!("{}", user1.username)
}
