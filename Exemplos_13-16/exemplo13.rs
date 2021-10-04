// slide 19 - 20
/*Suponha que a variável a seja uma inteira
sem sinal de valor 0x6DB7, como antes. Transforme a sua correspondente
representação binária em uma outra
representação binária na qual os 8 bits
mais à direita são todos 1s e os 8 bits
mais à esquerda permanecem com seus
valores originais. Assinale essa
representação binária à variável inteira
sem sinal b.*/

/* operação será? - b = a | 0xFF
Vamos examinar o resultado:
– a = 0110 1101 1011 0111 = 0x6DB7
– M = 0000 0000 1111 1111 = 0xFF
– b = 0110 1101 1111 1111 = 0x6DFF
*/

fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0xFF;
    let b:u16 = a | mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}