//slide 37 - 38
/*Operadores de Deslocamento
• Exemplo: – b = a >> 6
– Deslocará todos os bits de a seis posições à
direita e o resultado será atribuído à variável b.
– b = 0x1B6
– Vamos ver o resultado final
0 1 1 0 1 1 0 1 1 0 1 1 0 1 1 1
0 0 0 0 0 0 0 1 1 0 1 1 0 1 1 0
*/
fn main() {
   let a = 0x6DB7;
   let b = a >> 6;
   println!("a = {:016b}",a);
   println!("(a >> 6) => {:016b}",b);
   println!("Resultado de b em hexadecimal: {:#x}",b);
}