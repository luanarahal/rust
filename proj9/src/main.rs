#![allow(unused, dead_code)]

/*fn convert_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}   //PODE SER SUBSTITUÍDO PELA LINHA 17

fn double(n: i32) -> i32 {
    n * 2
}*/  //PODE SER SUBSTITUÍDO PELA LINHA 19

fn main() {
    let input = "56 65 58 48 59 56 87 23";

    let result: Vec<i32> = input
                .trim()
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                //map(convert_to_number)  SÓ USARIA SE TIVESSE A FUNÇÃO HABILITADA
                .map(|n| n * 2)
                .collect();
    println!("{:?}", result)
}
