//slide 35 - 36
/*Operadores de Deslocamento
• Exemplo: – b = a << 6
– Deslocará todos os bits de a seis posições à
esquerda e o resultado será atribuído à variável b.
– b = 0x6DC0
– Vamos ver o resultado final
O 1 1 0 1 1 0 1 1 0 1 1 0 1 1 1
0 1 1 0 1 1 0 1 1 1 0 0 0 0 0 0
*/
fn main() {
    let a = 0x6db7;
    let b:u16 = a << 6;
    println!("- b = {:016b} = {:#x}", b, b);
}