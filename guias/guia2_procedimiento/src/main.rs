const IVA: f64 = 0.13;

fn ejercicio1(){
    let mut temperatura: f64 = 36.6;
    println!("Temperatura inicial: {} °C", temperatura);

    temperatura = 37.0;
    println!("Temperatura actualizada: {} °C", temperatura);
}

fn ejercicio2(){
    let precio = 100.0;
    println!("Precio base: ${:.2}", precio);

    let precio = precio * (1.0 + IVA); //Shadowing con nuevo tipo igual
    println!("Precio con IVA ({}%): ${:.2}", IVA * 100.0, precio);
}

fn ejercicio3(){
    println!("Máximo u8:{}", u8::MAX);
    println!("Mínimo i32: {}", i32::MIN);
    println!("Máximo i32: {}", i32::MAX);
    println!("Máximo u64: {}", u64::MAX);
    println!("Tamaño de usize en este sistema: {} bits", usize::BITS);
}


fn main() {
    //ejercicio1()
    //ejercicio2()
    ejercicio3()
}
