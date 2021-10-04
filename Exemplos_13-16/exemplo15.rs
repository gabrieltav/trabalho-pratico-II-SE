// slide 23 - 24
/*Suponha que a seja uma variável inteira
sem sinal cujo valor é 0x6DB7, como nos
exemplos anteriores. Vamos agora
inverter os 8 bits mais à direita e preservar
os 8 bits mais à esquerda. Essa nova
representação binária será assinalada à
variável inteira sem sinal b. 
• Para isso vamos utilizar a expressão:
 - b = a ^ 0xFF, portanto:
 - b = 0x6D48

 Máscara

• Vamos verificar a validade: 
– a = 0110 1101 1011 0111 = 0x6DB7
– M = 0000 0000 1111 1111 = 0xFF
– b = 0110 1101 0100 1000 = 0x6D48
*/
fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0xFF;
    let b:u16 = a ^ mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}