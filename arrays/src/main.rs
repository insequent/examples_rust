fn main() {
    let array: [i32; 3] = [1, 2, 3];

    for i in array {
        println!("{}", i);
    }

    println!();

    let array: [&str; 5] = ["Mr.Meeseeks"; 5];

    for s in array {
        println!("I'm {}!! Look at me!", s);
    }
}
