//slide 28 - 29
/*Máscara
• E se eu quiser alterar alguns bits para um
valor pré-estabelecido?
• Por exemplo, eu quero trocar os dois bits
menos significativos para o valor “10”.
B = ((B & M1) | M2)
*/
fn main() {
   let m1 = 0xFC;
   let m2 = 0x02;
   let b = 0xAF;
   let x = b & m1;
   let z = x | m2;
   println!("{:#x} = {:08b} = ((B & M1) | M2)",z,z);
}