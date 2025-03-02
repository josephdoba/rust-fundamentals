fn main() {
    println!("This is the guide to ownership in Rust");
    println!(
        "therefore, the following is valid ownership since its within the brackets of our main()"
    );

    let owner1 = String::from("Hello!");
    let owner2 = owner1.clone();

    println!("{}", owner1);
    println!("{}", owner2);
}
