// slide 5
// Operador complementar (32 bits)
fn main() {
    let a = 0x5B3C;
    let complemento :u32 = !a;
    println!("Representação em bits: {:#018b}", a);
    println!("Representação em hexadecimal: {:#x}", a);
    println!("Complemento em bits: {:#b}", complemento);
    println!("Complemento representação em hexadecimal: {:#x}", complemento);
    println!("Ou seja: ~{:#x} = {:#x}", a, complemento);
}