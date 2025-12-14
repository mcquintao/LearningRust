fn main() {
    println!("Hello, world!");
    another_function();
    function_with_params(5, 10);
    print_labeled_measurement(42, 'm');

    //Statements and Expressions | Declarações e Expressões
    let y = 6; // Isto é um statement (declaração)
    
    // let x = (let y = 6); // Isto é inválido porque declarações não retorna um valor

/*
    // Blocos de código também são expressões
*/

    let x = {
        let y = 6; // Isto é um statement dentro do bloco
        y + 1      // Isto é uma expressão que retorna um valor (Finaliza sem ponto e vírgula)
    }; // O valor de x será 7

    // Ao adicionar um ponto e vírgula ao final, transforma a expressão em uma declaração (statement)

    println!("The value of five is: {}", five());
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Retorna o valor de x + 1
}

fn five() -> i32 {
    5 // Retorna o valor 5
}

fn another_function() {
    println!("This is another function.");
}

fn function_with_params(x: i32, y: i32) {
    println!("X + Y = {}", x + y);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}