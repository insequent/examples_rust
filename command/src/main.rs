use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("/")
        .output()
        .expect("No worky");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
