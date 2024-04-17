extern crate rand; // puxa uma lib(dependencia) externa que não tenha por padrao no rust

use std::io; // lib
use std::cmp::Ordering; // lib
// "Rng" --> trait que define métodos a serem implementados pelos geradores de numeros aleatorios
use rand::Rng; // lib

fn main() {
    println!("Guess the number!");

    // thread_rng iniciaza uma thead concorente local
    // entao chamamos o gen_range para gerar o numero aleatorio
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new(); // define que o numero é mutavel mut

        // o read_line retorna um tipo io::Result e o expect trata oq foi retornado
        io::stdin().read_line(&mut guess).expect("Error read entry");
        // transforma guess em um numero sem sinal (u32) o trim elimina espacos em branco
        // da string e o parse transforma ela em u32, o match esta ali como controle de erro.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue
            },
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Secret number is: {}", secret_number);
                println!("OK");
                break;
            },
        }
    }

}
