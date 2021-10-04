// slide 8
/*Suponha que a e b sejam variáveis
inteiras sem sinal de valores 0x6DB7 e
0xA726*/
// Qual o valor de ~a ?
fn main() {
    let a = 0x6DB7;
    let complemento :u16 = !a;
    println!("Representação de 'a' em bits: {:#018b}", a);
    println!("Valor de ~a: = {:#b} = {:#x}", complemento, complemento);
}