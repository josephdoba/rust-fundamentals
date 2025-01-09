fn main() {
    // prints:
    print!("this is \"print!\"");
    println!("this is \"println!\" ");
    println!("this is \"println!\" too ");
    eprint!("this is \"eprint!\"");
    eprintln!("this is \"eprintln!\"");
    // can string interpolate directly within the arguments. Interesting!
    println!(
        "{word1} {word2} {word3}",
        word1 = "the",
        word2 = "big",
        word3 = "dog"
    );

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James" -- fixed

    // Primitives

    let num1_float: f64 = 4.1; // regular notation
    let num2_int = 5i32; //Suffix notation

    //if undefined, defaults will be used: f64 and i32
    let def_float = 3.2;
    let def_int = 9;

    println!(
        "num1 = {}, num2 = {}, num3 = {}, num4 = {}",
        num1_float, num2_int, def_float, def_int
    );
}
