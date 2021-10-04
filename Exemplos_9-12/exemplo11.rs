// slide 13 - 15
/*Suponha que a=0x6DB7 e que queiramos extrair os 6
bits mais a direita de a e assinalar à variável b. Como
fazer isso?*/

fn main() {
    let a:u16 = 0x6DB7;
    let mascara:u16 = 0x3F;
    let b:u16 = a & mascara;
    println!("- a = {:016b} = {:#x}", a, a);
    println!("- M = {:016b} = {:#x}", mascara, mascara);
    println!("- b = {:016b} = {:#x}", b, b);
}