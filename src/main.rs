use std::io;
struct Rectangle {
    width: u32,
    height: u32,
}

// This method makes the code clearer and it gives us more flexibility.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn halve(&self) -> u32 {
        // Here we are squaring it.
        (self.width * self.height) / 2
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Please enter the width.");
    let mut input_width = String::new();

    io::stdin()
        .read_line(&mut input_width)
        .expect("Line was empty.");

    let input_width: u32 = input_width.trim().parse().expect("Please enter a number.");

    println!("Please enter the height.");
    let mut input_height = String::new();

    io::stdin()
        .read_line(&mut input_height)
        .expect("Could not read line");

    let input_height: u32 = input_height.trim().parse().expect("Please enter a number.");

    let rect1 = Rectangle {
        width: input_width,
        height: input_height,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 100,
    };

    println!(
        "\nThe Rectangle dimensions are: {} square pixels",
        rect1.area()
    );

    println!("\nIf we half it, our dimentions will be: {}", rect1.halve());

    if rect1.width() {
        println!(
            "\nThe rectangle has a nonezero width; it is {}\n",
            rect1.width
        );
    };

    println!("Can rect1 hold rect2? {}\n", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}\n", rect2.can_hold(&rect1));
}
