//slide 30 - 31
/*Máscara
Outro exemplo, eu quero trocar os bits 5,4
e 3 para o valor “101” e mantendo os
outros bits intactos. – B = (( B & M1) | M2) – Onde M1=11100011 e M2=00010100
*/
fn main() {
   let m1 = 0xE3;
   let m2 = 0x14;
   let b = 0xAF;
   let x = b & m1;
   let z = x | m2;
   println!("{:#x} = {:08b} = ((B & M1) | M2)",z,z);
}