const TRES_HORAS_EM_SEGUNDOS : u32 = 3 * 60 * 60;

fn main() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");
    println!("Três horas tem {} segundos", TRES_HORAS_EM_SEGUNDOS);


    let y = 5;
    let y = y + 1;  //Shadow -> Uma nova variável de mesmo nome é criada

    { 
        let y = y * 2; // Novamente uma 3a variável é criada mas somente neste contexto
    }
        println!("O valor de y é {y}"); // Fora do contexto, a variável volta a ser a anterior (y = y+1 = 6)
}
