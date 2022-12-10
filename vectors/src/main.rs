fn main() {
    let collected: Vec<i8> = (1..7).collect();
    println!("Vector as a collection: {:?}", collected);

    let mut vec = vec!["This"];
    vec.push("is");
    vec.push("a");
    vec.push("vector");

    println!("{}!", vec.join(" "));
}
