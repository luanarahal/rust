fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase()                     //* é para referenciar 
}

fn add_prefix(text: &mut String) {
    //*text = format!("FOO_{text}")
    text.push_str("_FOO");
}

fn main() {
    let mut name = "Luana".to_string();
    to_uppercase(&mut name);                         //empréstimo
    add_prefix(&mut name);                           //empréstimo
    println!("{name}")
}
