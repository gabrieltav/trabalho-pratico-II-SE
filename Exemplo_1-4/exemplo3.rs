//0x1111
//Operador complementar 32 bits\par
fn main(){
    let a = 0x1111;
    let complemento :u32 = !a;
    println!("Representação em bits: {:#018b}", a);
    println!("Representação em hexadecimal: {:#x}", a);
    println!("Complemento em bits: {:#b}", complemento);
    println!("Complemento representação em hexadecimal: {:#x}", complemento);
    println!("Ou seja: ~{:#x} = {:#x}", a, complemento);
}
