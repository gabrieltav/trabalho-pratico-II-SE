// slide 21 - 22
/*Suponha, agora, que desejamos
transformar a representação binária de
a em outra representação binária, na
qual os 8 bits mais à esquerda são
todos 1s e os 8 bits mais à direita
permanecem com os seus valores
originais.*/

/*Máscara
• A operação será:
– b = a | 0xFF00

• Vamos examinar o resultado:
– a = 0110 1101 1011 0111 = 0x6DB7
– M = 1111 1111 0000 0000 = 0xFF00
– b = 1111 1111 1011 0111 = 0xFFB7 */

fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0xFF00;
    let b:u16 = a | mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}