fn say_hello(name: &str, color: &str) {                 //fn recebendo duas strings 
    println!("Hello {name}, your color is {color}");
}

fn add_numbers(x: i32, y: i32) -> i32 {                 //fn recebendo dos números inteiros e somando
    if x == 0 {
        return y                                        //se x == 0 ele não irá retornar x + y, será um early return, e retornará apenas y
    }
    x + y                                               //ou pode escrever:     return x + y;
}

fn main() {                                             //chamando a função say_hello com os parâmetros
    say_hello("Luana", "pink");
    say_hello("Adriano", "blue");

    let y = {                                           //utilizado quando não tem valor de retorno (nulo)
        say_hello("Luana", "pink");
        99                                              //colocando esse 99, ele considera que é y
    };

    println!("{:?}", y);                                //imprimindo y -> irá retornar () por default

    let res = add_numbers(8, 9);                        //chamando a função add_numbers e passando 2 parâmetros
    println!("{res}");
}