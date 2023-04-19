use std::io;

fn main() {
    let mut s = String::new();
    println!("Digite um texto");

    io::stdin()
        .read_line(&mut s) //result
        .expect("Error reading console");
    
    println!("Você digitou: {s}");
    println!("{}", "-".repeat(40));
    println!("Quantidade de letras - byte: {}", s.trim().len());                //se fizer assim, quando tiver emojis ou caracteres especiais, irá contar errado, pois conta por byte 
    println!("Quantidade de letras - caractere: {}", s.trim().chars().count()); //aqui está correto pois irá contar por caractere

    println!("{}", "-".repeat(40));                                             //repetindo 40x o hífen -
    println!("Colocando tudo em maiúsculo: {}", s.to_uppercase());              //colocando tudo em maiúsculo
    println!("Colocando tudo em minúsculo: {}", s.to_lowercase());              //colocando tudo em minúsculo
    println!("{}", "-".repeat(40));                                             //repetindo 40x o hífen -
    println!("Substituíndo a letra n por x: {}", s.replace("n", "x"));          //substituindo valores dentro da string
}
