// 0xFFFF\par
//Operador complementar 32 bits\par
fn main() { 
    let a = 0xFFFF;
    let complemento :u32 = !a;
    println!("Representação em bits: {:#018b}", a);
    println!("Representação em hexadecimal: {:#x}", a);
    println!("Complemento em bits: {:#b}", complemento);
    println!("Complemento representação em hexadecimal: {:#x}", complemento);
    println!("Ou seja: ~{:#x} = {:#x}", a, complemento);;
}
