struct Creatura {
    // String este o structură
    tip_animal: String,
    nume: String,
    brate: i32,
    picioare: i32,
    arma_de_aparare: String,
}

struct Locatie (i32,i32);

struct NormalStruct{
    a:i32,
    b:i32,
}

struct TupluStruct(i32,i32);


fn main() {
    // Folosim o metodă statică ca să creem o instanță String
    let s = String::from("Metoda statica pentru o instanta String");
    println!("Static s = {}", s);
    println!("Static::{}, metoda.instanta={}",s,s.len());

    // datele din Creatur sunt pe stivă
    let ferris = Creatura{
        // Struct-ul String este de asemenea pe stivă,
        // dar ține o referință a informației pe heap
        tip_animal: String::from("Furr"),
        nume: String::from("Ferris"),
        brate:2,
        picioare:2,
        arma_de_aparare: String::from("Gheara"),
    };

    let berris = Creatura{
      tip_animal: String::from("Ursu"),
        nume: String::from("Berrys"),
        brate: 2,
        picioare: 2,
        arma_de_aparare: String::from("Colti"),
    };

    println!("Creatura 1: {}, {}, {},{},{}", ferris.tip_animal, ferris.nume, ferris.arma_de_aparare, ferris.brate, ferris.picioare);
    println!("Creatura 2: {}, {}, {}, {},{}.", berris.tip_animal, berris.nume, berris.arma_de_aparare, berris.brate, berris.picioare);
    // Așadar, cei doi prieteni ai noștri, Ferris și Berris, sunt structuri de date care vor avea mereu locații fixe în aplicația noastră, deci vor fi puși în stivă.


    // Structuri TUPLU
    let loc = Locatie(22,33);
    println!("Locatia exacta: {}, {}", loc.0, loc.1);

    let ns = NormalStruct{a:1,b:2};
    let ts = TupluStruct(3,4);

    // Atribuire si deconstruct in variabile
    let TupluStruct(x,y) = ts; // 3,4
    println!("x,y : {} {}", x,y); // 3,4

}
