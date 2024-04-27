# RUST DEVELOPER

### Compilar app en RUST
rustc hello_world.rs

TOML = Tom's Obvious Minimal Languaje

## CARGO GUIDE

### Creacion de una aplicacion
cargo new hello_cargo

### Creacion de una libreria
cargo new hello_cargo_lib --lib

### Compila proyecto
cargo build (dentro del proyecto creado)

### Estructura del proyecto
<p>D:.</p>
<p>├───src</p>
<p>└───target</p>
<p>&nbsp&nbsp&nbsp&nbsp└───debug</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp├───.fingerprint</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp│&nbsp&nbsp&nbsp&nbsp└───hello_cargo-c0fe7de074aed428</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp├───build</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp├───deps</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp├───examples</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp└───incremental</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp└───hello_cargo-2x6muc1a276jp</p>
<p>&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp└───s-gvlhdd10ij-1bum09r-530tly1xn5491fu8vs5cg3up6</p>
		
En target\debug\ se encuentra el ejecutable del proyecto

### Compila y ejecuta el proyecto
cargo run

### Verifica si la compilacion terminará ok
cargo check

### Compila y realiza optimizaciones
cargo build --release

### Para cargar dependencia
1. visitar https://crates.io
2. agrega dependencia
3. ejecutar run en proyecto

### Agregar Paquetes por cargo
<p>cargo add log (agrega paquete log)</p>
<p>cargo add serde --features derive (agrega paquete serde con feature derive)</p>
