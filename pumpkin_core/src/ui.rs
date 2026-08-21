use std::process::Command;

pub fn start(){

    let output = Command::new("bash")
    .arg("-c")
    .arg("figlet PUMPKIN | lolcat -f")
    .output()
    .expect("Failed to generate logo");

    let logo = String::from_utf8_lossy(&output.stdout);

    println!("{}", logo);
}