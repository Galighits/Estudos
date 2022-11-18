//Estudo de Variáveis

//fn main() {
//   let x = 5;
//    println!("o valor de x é: {}", x);
//    x = 6;
//    println!("o valor de x é {}", x);
//}

//error[E0384]: cannot assign twice to immutable variable `x`
 //--> src/main.rs:4:5
// |
//2 |     let x = 5;
// |         - first assignment to `x`
//3 |     println!("O valor de x é: {}", x);
//4 |     x = 6;
// |     ^^^^^ cannot assign twice to immutable variable

// erro acima que aparece ao usarmos o cargo run, só pra mostrar como o compilador nos ajuda a encotrar erros em nosso código.

fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
}

//ompiling variables v0.1.0 (/home/derra/Estudos/Rust/Estudo de conceitos/variables)
//Finished dev [unoptimized + debuginfo] target(s) in 4.28s
//Running `target/debug/variables`
//O valor de x é: 5
//O valor de x é: 6

//Quando usamo o mut, a ultima variavel altera a anterior.

//Shadowing 

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("O valor de x é: {}", x);
}

//ao rodarmos isso, o valor de x será 12, pois x é sombreado pelo de cima, entao ele pega o valor original é acrescenta 1, somando 6, no final ele multiplica esse valor.

