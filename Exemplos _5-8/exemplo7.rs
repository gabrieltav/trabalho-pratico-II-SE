// slide 9
/*Suponha que a e b sejam variáveis
inteiras sem sinal de valores 0x6DB7 e
0xA726*/
// Qual o valor de ~b ?
fn main() {
    let b = 0xA726;
    let complemento :u16 = !b;
    println!("Representação de 'b' em bits: {:#018b}", b);
    println!("Valor de ~b: = {:#b} = {:#x}", complemento, complemento);
}