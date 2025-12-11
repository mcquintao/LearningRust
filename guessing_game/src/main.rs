use rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número");

    let secret_number = rand::random_range(1..=100);

    loop {
        println!("Digite o número:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Erro ao ler a linha");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Digite apenas números inteiros!");
                    continue;
                }
            };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("é maior!"),
            Ordering::Greater => println!("é menor!"),
            Ordering::Equal => {
                println!("acretou miseravi!");
                break;
            }
        }
    }
}
