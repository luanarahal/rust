fn main() {
    let a: i32 = 1; //Copy (i32, f64, bool, char)
    let b = a;      //Faz uma cópia de A, mas é completamente independente
                    //se fosse let b = &a, B estaria apontando para A, mas quando fosse referenciar, teria que usar *b na linha 6
    println!("O valor de A é: {a}");
    println!("O valor de B é: {b}");

    let c = String::from("Luana");  //No Copy - criando a variável c, na memória HEAP
    let d = c;                      //neste caso estará MOVENDO a propriedade de C para D e INVALIDANDO a variável C. C não poderá mais ser imprimida.
                                    //pode ser resolvido com let d = &c, neste caso, C estará EMPRESTANDO a variável 
}

//REGRAS DE OWNERSHIP EM RUST

//1 Cada valor tem um dono (owner)
//2 Só pode ter um único dono
//3 Quando o dono sai de escopo, o valor é limpo
//4 A posse (ownership) pode ser movida a outro dono
