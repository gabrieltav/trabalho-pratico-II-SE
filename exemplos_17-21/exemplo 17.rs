//slide 26 - 27
/*Suponha que a seja uma variável inteira sem sinal
cujo valor é 0x6DB7, como nos exemplos
anteriores. • Qual o resultado da expressão a ^ 0x4?
– inverterá o valor do bit número 2 (o terceiro bit a partir da
direita) de a. • Se essa operação for executada repetidamente, o
valor de a será alternado entre 0x6DB7 e 0x6DB3
• Nesse caso, o terceiro bit será “ligado” e
“desligado” alternadamente.

Máscara

• Vamos verificar a validade: – a = 0110 1101 1011 0111 = 0x6DB7
– M = 0000 0000 0000 0100 = 0x4
– b = 0110 1101 1011 0011 = 0x6D48
– M = 0000 0000 0000 0100 = 0x4
– c = 0110 1101 1011 0111 = 0x6D48
*/
fn main() {
   let a = 0x6db7;
   let m = 0x4;
   let b = a ^ m;
   println!("- a = {:016b} = {:#x}", a, a);
   println!("- M = {:016b} = {:#x}", m, m);
   println!("- b = {:016b} = {:#x}", b, b);
}