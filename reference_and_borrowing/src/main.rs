fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");

    // say_hello(m1, m2); // m1 e m2 passam a ser propriedade de a e b 
                            // no escopo de sa_hello.
                            // Ao retornar, a e b são destruidos.
    
    let s = format!("{} {}!", m1, m2); //não seia possivel usar m1 e m2 aki...
    println!("{s}");
    say_hello_again(&s);
    println!("{s}");

    let mut x : Box<i32> = Box::new(1);
    let a = *x; //i32;
    *x += 1;
    // a -> 1 | a derreferencia x, pegando seu valor (1);
    println!("a = {a}, x = {x}");


    let r1 = &x; //&Box<i32> | referencia para x
    let b = **r1; // ** derreferencia para 1;

    let r2 = &*x; // &i32 | cria uma referencia para i32 (*x);
    let c = *r2; // i32 | derreferencia a referencia acima;

    // Derreferenciando implicitamente (.) e explicitamente...
    let x : Box<i32> = Box::new(-1);
    let x_abs1 = x.abs(); // Tem que tipar a variavel (?);
    let x_abs2 = i32::abs(*x);
    assert_eq!(x_abs1, x_abs2);


}

fn say_hello(a : String, b : String) {
    println!("{} {}!", a, b);
}

fn say_hello_again(c : &String) {
    println!("{c} (again)");
}