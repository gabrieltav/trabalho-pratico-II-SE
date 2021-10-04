// slide 11
/*Suponha que a e b sejam variáveis
inteiras sem sinal de valores 0x6DB7 e
0xA726*/
// Qual o valor de a ^ b ?
fn main() {
    let a = 0x6DB7;
    let b = 0xA726;
    let result:u16;
    result = a ^ b;
    println!("Representação de 'a' em bits: {:#018b}", a);
    println!("Representação de 'b' em bits: {:#018b}", b);
    println!("a ^ b = {:#b} = {:#x}", result, result);
}