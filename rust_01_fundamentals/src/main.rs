//--- DEFINICION DE CONSTANTES
/* Las constantes se definen con la palabra const
 * Se puden definir en cualquier scope (hasta fuera del main)
 * Son inmutables y no permiten shadowing
 */
const NRO_ESPACIOS:i32 = 23;

fn main() {
    println!("El valor de NRO_ESPACIOS es: {}", NRO_ESPACIOS);

    //--- VARIABLES INMUTABLES
    let x:u8 = 10;
    // x = 5; por defecto las variables son inmutables
    println!("El valor de nuestra variable 'x' es: {}",x);
    let mut y:u8 = 11; // para hacer mutable una variable se usa "mut"
    println!("El valor de nuestra variable 'y' es: {}",y);
    y = 5;
    println!("El valor de nuestra variable 'y' modificada es: {}",y);

    //--- SHADOWING
    // Con shadowing se permite cambiar el tipo de variable
    // espacios es una variable cadena
    let espacios = "   ";
    println!("El usuario ingresó: {}", espacios);
    // espacios es una variable numerica
    let espacios = espacios.len();
    println!("El número de espacios es {}", espacios);

    //--- TIPOS DE DATOS
    // Tipado Estático: chequeo de los tipo de datos en tiempo de compilación ==> RUST
    // Tipado Dinámico: chequeo de los tipo de datos en tiempo de ejecución ==> PYTHON
    // Integer
    let entero: i8 = 23; // Signed de 8 bits
    let entero2: u8 = 40; // Unsigned de 8 bits
    let entero3: i8 = -40;

    // Integer Literal
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    // Floating point
    let float1 = 5.0; // f64
    let float2: f32 = 12.432; // f64

    // Boolean
    let verdadero = true;
    let falso = false;

    // Caracter - Usa Unicode un set de simbolos mas extenso
    let caracter = 'a';
    let emoji = '🎉'; 

    //// Compund Types
    
    // Tuplas
    // Las tuplas tienen tamaño fijo no se pueden modificar
    // Todos los elementos no son del mismo tipo de dato
    let tupla = ('h', 23, -45, 0.222);
    let tupla2: (char, u64, i32, f64) = ('h', 23, -45, 0.222);
    let (x, y, z, w) =  tupla;
    println!("el valor de x es: {}", x);
    println!("el segundo valor de la tupla es: {}", tupla.1);

    // Array
    // Los array tienen tamaño fijo no se pueden modificar
    // Todos los elementos son del mismo tipo de dato
    let arreglo = [1, 2, 3, 4, 5]; // La longitud y el tipo es implicito
    println!("el segundo valor del arreglo es: {}", arreglo[1]);
    let arreglo2: [i128; 5] = [1, 2, 3, 4, 5]; // La longitud y el tipo es explicito

    // String
    let mut nombre: &'static str = "Aldo Pizarro";
    nombre = "Nombre Apellido";

    let apellido: String = "Pizarro".to_string();
    let mut domicilio = String::new();
    domicilio = "mi casa".to_string();

}
