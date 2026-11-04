/// Funcion para imprimir un saludo en la salida estandar
///
/// # Argumentos
/// * `nombre` - El nombre de la persona a saludar
///
/// # Ejemplo
/// let nombre = "Dominic";   
/// saludar(nombre);   
/// //Imprime: Hola Dominic, te saludo desde una función."
fn saludar(nombre: &str){
    println!("Hola {}, te saludo desde una función.", nombre);
}

fn main() {
    let nombre = "Dominic";
    let edad = 19;
    let sistema = "NixOS (Linux)";

    let x = true;

    println!("Hola soy {}, tengo {} años de edad y estoy programando desde {}.", nombre, edad, sistema);

    saludar(nombre);

    if x {
        println!("x es verdadero");
    }
}