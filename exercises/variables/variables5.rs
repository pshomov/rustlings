// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number2 = "3";
    println!("Number plus two is : {}", number2.parse::<i8>().unwrap() + 2);
}
