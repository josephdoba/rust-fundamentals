

fn main() {
// Defining Methods:
struct Rectangle {
    width: u32,
    height: u32, 
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("The area of the rectangle is: {}", rect1.area())

// neat! just like js methods


}
