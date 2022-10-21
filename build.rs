use std::process::Command;

fn main() {
    if cfg!(not(debug_assertions)) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("npm run build -w front")
            .output()
            .unwrap();
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}
