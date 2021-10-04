// slides 25 - 26
/*Máscara

Se quisermos inverter os 8 bits mais à
esquerda de a e manter os 8 bits mais
à direita originais:
b = a ^ 0xFF00

a = 0110 1101 1011 0111 = 0x6DB7
M = 1111 1111 0000 0000 = 0xFF00
b = 1001 0010 1011 0111 = 0x92B7*/

fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0xFF00;
    let b:u16 = a ^ mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}