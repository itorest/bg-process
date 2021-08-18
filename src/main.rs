use std::process::{Command, Stdio};

fn main() {
    if let Ok(mut c) = Command::new("ls")
        .arg("-l")
        .stdout(Stdio::null())
        .spawn() {
        // if let Ok(mut c) = Command::new("sleep").arg("20").spawn() {
        println!("Spawned successfully");
        println!("{}", c.id());
        // println!("Exit with: {:?}", c.wait());
    } else {
        panic!("panic");
    }
}
