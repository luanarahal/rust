fn say_hello(text: &String) {
    println!("Hello, {text}")
}

fn say_goodbye(text: &String) {
    println!("Goodbye, {text}")
}

fn main() {
    let name = "Luana".to_string();
    say_hello(&name);               //empréstimo
    say_goodbye(&name);             //empréstimo
}
