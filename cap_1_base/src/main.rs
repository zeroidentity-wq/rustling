// Functii
fn add(a: i32, b:i32) -> i32 {
    return a + b;
}
fn substract(x:i32, y : i32) -> i32 {
    return x-y
}
// Returneaza un tuplu
fn swap(x:i32, y:i32) -> (i32, i32) {
    return (y,x);
}
// Functii care nu returneaza nimic
fn make_nothing() -> (){
    return ();
}

fn make_nothing2(){
    // nu face nimic
}

// Constante
const PI: f32 = std::f32::consts::PI;
fn main() {
    /*
     VARS
     Numele variabilelor sunt snake_case
     Rust intuiește tipul de date pentru x */
    let x = 13;
    println!("x: {}", x);
    // Rust poate fi explicit in declararea tipului
    let x : f32 = 3.35;
    println!("x32: {}", x);
    // Declarare si initializare ulterioara.
    let x;
    x = 335;
    println!("x: {}", x);

    // Constante ------------------------------------------------------
    println!(
        "Pentru a crea un măr {}, mai întâi trebuie să creezi un univers.",
        PI
    );

    // ----------------------------------------------------------------------
    // MODIFICAREA VARIABILELOR

    let mut var_mutabila = 5;
    println!("Variabila mutabila: {}", var_mutabila);
    let imutabil = 33;
    println!("Var imutabila: {}", imutabil);

    let x = 12; // acesta este un i32 în mod implicit
    let a = 12u8;
    let b = 4.3; // acesta este un f64 în mod implicit
    let c = 4.3f32;
    let d = 'r'; // caracter unicode
    let ferris = '🦀'; // tot un caracter unicode
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, d, ferris, bv, t.0, t.1, sentence
    );
    // ----------------------------------------------------------------------
    // CONVERSIE TIPURI
    // Rust poate face conversia de la un tip numeric la altul, foarte ușor, folosind cuvântul cheie as
    let c = a as u32;
    let b = c as i32 + a as i32;
    println!("b converted: {}", c);
    let y = 3.5;
    let mut nr_int = y as i8;
    println!("Float: {}, Integer: {}", y, nr_int);

    /* TABLOURI, FIXE, index 0, accesare [0]
    [TIP;NR] */
    let tablou: [i32;5] = [1, 2, 3, 4, 5];
    for i in 0..tablou.len() {
        print!("{:?} ", tablou[i]);
    }
    println!("Elemente tablou: {:?}", tablou);
    println!("Element[0] = {}", tablou[0]);

    println!("42 + 13 = {}", add(42, 13));
    println!("13-13 = {}", substract(13, 13));

    // Returneaza un tuplu de valori
    let rezultat = swap(35,89);
    println!("Rezultat SWAP: {:?}", rezultat);
    // Destructureaza Tuplu in 2 variable
    let(p1,p2) = swap(rezultat.0, rezultat.1);
    println!("Rezultat SWAP {},{}", p1, p2);

    // Returneaza nimic in variabile
    let nimic1 = make_nothing();
    let nimic2 = make_nothing2();
    // printam nimicul
    println!("Nimic1 Nimic2: {:?}, {:?}", nimic1, nimic2);

}

