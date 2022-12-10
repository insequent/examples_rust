fn main() {
    let string = "This is a random string!";

    let mut parts = string.split_whitespace(); // Return iterator
    for i in 1..6 {
        // unwrap() like this is a bit unsafe, but we know the exact string here
        println!("{}: {}", i, parts.next().unwrap());
    }
}
