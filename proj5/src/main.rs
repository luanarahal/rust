fn main() {
    //TUPLA
    let numbers: (u32, u32, u32) = (1, 2, 3);
    println!("{:?}", numbers); //imprimindo toda a tupla
    println!("{:?}", numbers.2); //imprimindo somente a posição 2

    let (a, b, c) = numbers; //irá colocar a tupla respeitando esse objeto
    println!("{:?}", a); //ai pode chamar desta forma
    println!("{:?}", b);
    println!("{:?}", c);

    let mut numbers2 = (1, 2, 3, true, 7.5); //assim consigo deixar a tupla mutável
    numbers2.0 = 50; //alterando a posição 0 para 50 ao invés de 1
    numbers2 = (4, 5, 6, false, 9.8); //alterando toda a tupla, mas precisa respeitar os tipos
    println!("{:?}", numbers2); 

    //ARRAY
    let mut array: [i32;3] = [1, 2, 3];
    println!("{:?}", array); //imprimindo todo o array
    println!("{:?}", array[2]); //imprimindo somente a posição 2
    array[1] = 5; //alterando o elemento da posição 1 para o número 5
    println!("{:?}", array[1]); //imprimindo somente a posição 1
    //println!("{:?}", array[10]); //irá dar erro porque não tem a posição 10

    //SLICE
    println!("{:?}", &array[1..3]) //para mostrar 1 e 2
    println!("{:?}", &array[1..]) //para começar a mostrar no 1 até o último
    println!("{:?}", &array[..2]) //para mostrar até a posição 2
}
