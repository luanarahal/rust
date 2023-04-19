use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");                 //colocando uma palavra no meio de hífens -----Calculadora-----
    let mut s = String::new();
    let banner = "Digite uma sequência de números\n\
    Separados por vírgula\n\
    Ex: 1, 2, 3, 4, 5";                                 //mostrando como escrever uma string separados em várias linhas
    
    println!("{banner}");
    println!("{}", "-".repeat(40));
    io::stdin()
        .read_line(&mut s) //result
        .expect("Error reading console");

    /*fn clean(c: &str) -> &str {                       //criando uma função para limpar os espaços e \n\b no final do vetor - pode ser substituída no .map aqui embaixo
        c.trim()
    }*/

    let nums: Vec<i32> = s                              //convertendo para número i32
        .split(",")                                     //separando por , 
        .map(|c| c.trim().parse().expect("Error"))      //podemos chamar a função clean em .map(clean) se tiver a função ali em cima
        .collect();                                     //transformando a string em um vetor para conseguirmos separar os números

    let result: i32 = nums.iter().sum();

    println!("{}", "-".repeat(40));
    println!("Você digitou: {:?}", nums);                //mostrando o vetor em forma de string
    println!("A soma de todos os valores é: {}", result) //somando os valores digitados
}
