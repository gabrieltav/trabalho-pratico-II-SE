// slides 16 - 17
/*Suponha, novamente, que a seja uma variável inteira
sem sinal de valor 0x6DB7. Agora, extraia os 6 bits mais
à esquerda desse valor e assinale à variável inteira sem
sinal b. Assinale 0s aos 10 bits mais à direita de b.*/

/*• Para executar essa operação podemos escrever a
expressão bitwise b=a & 0xFC00
• Assim, a constante 0xFC00 servirá como máscara
• O valor de b será 0x6C00.*/

/*Máscara
• b = a & 0xFC00
• A validade desse resultado pode ser verificada examinando-se as
representações binárias correspondentes 
– a = 0110 1101 1011 0111 = 0x6DB7
– M = 1111 1100 0000 0000 = 0xFC00
– b = 0110 1100 0000 0000 = 0x6C00
• A máscara agora bloqueia os 10 bits mais à direita de a.*/

fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0xFC00;
    let b:u16 = a & mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}