fn main() {
    // prints:
    print!("this is \"print!\"");
    println!("this is \"println!\" ");
    println!("this is \"println!\" too ");
    eprint!("this is \"eprint!\"");
    eprintln!("this is \"eprintln!\"");
    // can string interpolate directly within the arguments. Interesting!
    println!(
        "{obj} {subj} {verb}",
        obj = "the",
        subj = "big",
        verb = "dog"
    );

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    
}
