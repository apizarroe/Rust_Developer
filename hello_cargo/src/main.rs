/*
 * Esta es una importacion de librerias
 */
use rand::random;

/*
 * Esta función es el punto de entrada de un programa Rust
 */
fn main() {
    /* Se invoca a la funcion random() y se asigna el valor retornado a number*/
    let number:u8 = /*rand::*/random(); 
	println!("Hola mundo! tu numero es el: {}", number);
}
