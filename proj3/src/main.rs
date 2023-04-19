const SECONDS_IN_MINUTE: u32 = 60; //constantes são imutáveis, sempre em MAÍUSCULO 
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

//pode declarar const no escopo global
fn main() {
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos", total_em_segundos)
}
