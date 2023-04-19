fn main() {
    let x: u8 = 5; //declarando que a variável seja do tipo u8, eu estou economizando memória
    let y = 5_u8; //pode declarar dessa forma também
    //let z: u8 = x - 20; //irá dar erro porque u8 não aceita número negativo, dai ele prevê o erro
    let a: i8 = 5;
    //let b: i8 = a * 200; //irá dar erro porque u8 não aceita número tão grande -caracteres-, dai ele prevê o erro
    let c = 199_457_249; //pode criar um número gigante, separando com _ como se fosse , para ficar melhor a elegibilidade
    let h = 0xff; //base hexadecimal
    let o = 0o77; //base octal
    let b = 0b1111_0000; //base binário
    let by = b'A'; //byte
}
