fn main() { //escopo
    let mut total: i32 = 30; //declarando o tipo
    println!("Trabalhou {} horas", total);
    total = 44;
    println!("Trabalhou {} horas", total); //só consegue alterar a variável se utilizar let mut (mutável) -- e se iniciar com um tipo, só pode utilizar o mesmo tipo. Ex: iniciou com inteiro, depois não pode ser número quebrado ou string. A não ser que declare let denovo, igual irei fazer abaixo:
    //let total = "quarenta";
    //println!("Trabalhou {} horas", total);
    {
        let total = total + 20; //aqui está declarando uma variável local, mas utilizando o total de fora
        println!("Trabalhou {} horas", total);
    }
} //fim
//se declarar uma variável e não utilizar, no cargo run irá dar um warning 
//não pode declarar uma variável global
//drop

