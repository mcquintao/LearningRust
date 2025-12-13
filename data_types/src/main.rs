fn main() {
    println!("Data types!");

    let number : u32 = "42".parse().expect("Not a number");
    // Se não tiver a notação do tipo ": u32" o parse retornará um erro


//  |size    |unsig |sig
//  |8-bit	 |i8	|u8
//  |16-bit	 |i16   |u16
//  |32-bit	 |i32   |u32
//  |64-bit	 |i64   |u64
//  |128-bit |i128	|u128
//  |archi   |isize	|usize

// Padrão int = i32

    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32

    //Operations
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let mult = 4 * 30;
    let div = 56.7 / 32.5;
    let trunc = -5 / 3; // = -1
    let rem = 43 % 5; // = 3

    //Bool
    let t = true;
    let f : bool = false;

    //Char
    let c = 'z';
    let z : char = 'ℤ';
    let heart_eyed_cat = '😻';

    //Tuple
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let one = tup.2;
    println!("500 : {}; 1 : {}; 6.4 : {}", five_hundred, one, tup.1);
    // a tuple vazia () se chama 'unit' e representa um valor vazio ou tipo de retorno vazio.
    // Expressões retornam implicitamente um 'unit' se não retornarem outro valor.

    let mut mutTup : (i32, i32) = (1, 2);
    println!("{}, {}", mutTup.0, mutTup.1);
    mutTup.0 = 0;
    mutTup.1 += 5;
    println!("{}, {}", mutTup.0, mutTup.1);


    // Array
    //
    
    // Guarda um mesmo tipo;
    // Tem tamano fixo (fica na memória stack) | diferente do vector

    let array = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // Anotado array
    let ai32 : [i32; 5] = [1, 2, 3, 4, 5];

    let first = ai32[0];
    let last = ai32[4];
    let repet = [3; 5]; // [3, 3, 3, 3, 3]

    //quizz
    let t1 = ([1, 2], [3; 4]);
    // [1, 2]; [3, 3, 3, 3];
    let (a, b) = t1;
    
    println!("{} + {} = {}", a[0], t1.1[1], a[0] + t1.1[1]);
    // = 4

}
