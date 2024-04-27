# RUST

### Compilar app en RUST
rustc hello_world.rs

TOML = Tom's Obvious Minimal Languaje

CARGO
### Creacion de una aplicacion
cargo new hello_cargo

### Creacion de una libreria
cargo new hello_cargo_lib --lib

### Compila proyecto
cargo build (dentro del proyecto creado)

### Estructura del proyecto
D:.
├───src
└───target
    └───debug
        ├───.fingerprint
        │   └───hello_cargo-c0fe7de074aed428
        ├───build
        ├───deps
        ├───examples
        └───incremental
            └───hello_cargo-2x6muc1a276jp
                └───s-gvlhdd10ij-1bum09r-530tly1xn5491fu8vs5cg3up6
		
En target\debug\ se encuentra el ejecutable del proyecto

### Compila y ejecuta el proyecto
cargo run

### Verifica si la compilacion terminará ok
cargo check

### Compila y realiza optimizaciones
cargo build --release

Para cargar dependencia
1. visitar https://crates.io
2. agrega dependencia
3. ejecutar run en proyecto

Agregar Paquetes por cargo
cargo add log (agrega paquete log)
cargo add serde --features derive (agrega paquete serde con feature derive)
