## Notes

### & &mut
`&` `&mut` - **NU** returneaza RAW pointer, ci **REFERINTE**;  

`&mut`
* Singura **REFERINTA** care **DATELE** care **POINTEAZA**;
* Doar 1 **REFERINTA**;
* **MODIFY SAFE**;

> & != &mut 
* & - >1 REFERINTE catre DATA;

### Tuples
> **GRUPEAZA** mai multe DT de tipuri **DIFERITE**;

### Creare TUPLE
```rust
let persona_cu_type : (&str,i32,f64) = ("Tony", 27, 3,5);
let persona_fara_type = ("Tony", 27, 3.5); 

```
### Decompresarea unui tuplu
```rust
let persoana_cu_type : (&str,i32,f64) = ("Tony",3,3,5);
let (nume, varsta, inaltime) = persoana_cu_type;
```

### Mutabilitatea
```rust
// Mutabilitate
let persoana_imutabila = ("Imutabil", 1, 1.0);
// persoana_imutabila.1 = 1;  cannot mutate
let mut persoana_mutabila = ("Mutabil", 1, 3.5);
println!("Inainte DE SCHIMBARE: {:?}", persoana_mutabila);
persoana_mutabila.0 = "Am schimbat";
println!("Dupa SCHIMBARE: {:?}", persoana_mutabila);
```

### Ownership

* `i32`, `integer` - implementeaza `trait COPY`, astfel **valoarea este COPIATA**;
* `String` - **NU** implementeaza `trait COPY` , astfel **valoarea este MUTATA**;
```rust
    // Proprietatea
    let tuplu_copy = (42, "Tuplu Text".to_string()); 
    println!("tuplu_copy: {:?}",tuplu_copy);
    // # i32 implementeaza trasatura COPY;
    // # String NU implementeaza COPY, asa ca-l MUTA in "text"
    let (nr, text) = tuplu_copy;
    println!("Nr copiat in tuplu - nr :  {}", tuplu_copy.0); 
    // print!("tuplu_copy.1 : {}", tuplu_copy.1); # String nu implementeaza trasatura COPY;
    println!("tup.1 este mutat in text: {}", text);
```

### Tuplu ca si parametrul unei functii

```rust
    /*  
        * IF ALL elementele tuplului implementeaza COPY => tuplul poate fi copiat;
        * IF ALL elementele tuplului implementeaza COPY => NU va TRASNFERA OWNERSHIP atunci este pasat unui functii FARA a folosii 1 REFERINTA;
********************************************************************************************************
        * IF pasezi o REF catre tuplu, atunci NU TRANSFERA OWNERSHIP; 
        * IF >=1 element in tuplu este NON-COPY, OWNERSHIP este transferat atunci cand este pasat unei functii FARA a folosii o REFERINTA;
    */

    /*
        IF tuplu_non-COPY-> fn x(tuplu: &(i32, String)) -> x(&tuplu_non-COPY)
        IF tuplu_COPY - > fn y(tuplu: (i32, i32)) -> y(tuplu_COPY);

     */
    // NON-COPY
    let tuplu_non_copy : (i32,String)= (10, "NOT COPY".to_string());
    afiseaza_referinte_tuplu(&tuplu_non_copy); // PRIMESTE 1 REFERINTA;

    fn afiseaza_referinte_tuplu(tuplu: &(i32,String)) { //VA primi 1 REFERINTA;
        println!("Afiseaza referinte tuplu: {}, {}", tuplu.0, tuplu.1);
    }

    // COPY
    let tuplu_copy:(i32,i32) = (10,50); 
    fn afiseaza_copy_tuplu (tuplu:(i32,i32)){
        println!("Afiseaza COPY TUPLU: {} {}", tuplu.0, tuplu.1);
    }
    afiseaza_copy_tuplu(tuplu_copy); // VA PRIMI 1 COPIE; APEL FARA REFERINTA;

    // OWNERSHIP DE LA NON-COPY cu apel FARA REFERINTA
    fn afiseaza_tuplu_ownership(tuplu_non: (i32, String)){
        println!("Afiseaza OWNERSHIP Tuplu NON-COPY cu apel fara REF: {} {}", tuplu_non.0, tuplu_non.1);
    }
    afiseaza_tuplu_ownership(tuplu_non_copy);


```
## Capitol 1 - Base
### CONSTANTE
> **SNAKE_CASE**    
> Spre deosebire de variabile, constantelor trebuie să li se specifice explicit tipul la declarare.

```rust
const PI: f32 = 3.14159;

fn main() {
    println!(
        "Pentru a crea un măr {}, mai întâi trebuie să creezi un univers.",
        PI
    );
}

```

### Variabile

> `let`-> Rust poate in 99% din cazuri sa auto-atribuie datatype.
> Numele variabilelor sunt `snake_case`

```rust
fn main() {

    // Rust intuiește tipul de date pentru x
    let x = 13;
    println!("x: {}", x);
    // Rust poate fi explicit in declararea tipului
    let x : f32 = 3.35;
    println!("x32: {}", x);
    // Se poate declara o variabila si se poate initializa mai tarziu
    let x;
    x = 335;
    println!("x: {}", x);
}
```

### Modificarea Variabilelor
* **mutabile** (mutable) -> Compilatorul lasa userul sa modifice valoarea var.  
* **imutabile** -> Compilatorul lasa userul doar sa citeasca valoarea.
> Valorile **mutabile** sunt declarate cu `mut` 

```rust
fn main () {
let mut var_mutabila = 5;
    println!("Variabila mutabil: {}", mutabil);
let imutabil = 33;
    println!("Var imutable: {}", imutabil);
}
```

### Tipuri de date de baza

* **variabilă booleană** - `bool` pentru a reprezenta **adevărat** și **fals**
numere întregi fără semn - 
* `u8 u16 u32 u64 u128` pentru a reprezenta numere naturale -> **Unsigned**

* **numere întregi cu semn** - `i8 i16 i32 i64 i128` pentru a reprezentare numere întregi

* **numere întregi de dimensiunea unui pointer** - `usize isize` pentru a reprezenta indici și dimensiunea datelor în memorie

* **numere cu virgulă mobilă** - `f32 f64` pentru a reprezenta numere reale.

* **caractere** - `char` pentru reprezentarea unui singur caracter **Unicode**

* **tuplu** - `(valoare, valoare, ...)` pentru trecerea unor secvențe fixe de valori pe **stivă**

* **tablou** - `[valoare, valoare, ...]` o colecție de elemente de **același tip**; dimensiunea colecției este fixă și devine cunoscută doar în momentul compilării

* **parte (slice)** - o parte dintr-o colecție de elemente de același tip; dimensiunea părții devine cunoscută doar în timpul rulării  
* `str` **(string slice)** - text de lungime cunoscută în timpul rulării

```rust
fn main() {
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
}

```
### Conversia tipurilor de baza

>Rust poate face **conversia de la un tip** numeric la altul, foarte ușor, folosind cuvântul cheie `as`.

```rust
fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}
```

### Tablouri
> Un tablou este o **colecție de dimensiune fixă** de elemente care conțin **date de același tip**.

> Tipul de date pentru un tablou este scris sub forma `[T;N]`, unde `T` reprezintă **tipul** elementelor, iar `N` reprezintă **dimensiunea** **fixă** cunoscută la momentul compilării.

> Elemente **individuale pot fi accesate** cu ajutorul operatorului `[x]`, unde `x ` este un **index** `usize` (începând cu 0) al elementului pe care doriți să-l accesați.

> **Colecțiile cu dimensiune dinamică**, deseori numite **tablouri dinamice** sau variabile, vă vor fi prezentate într-un capitol viitor numit `Vectori`.

```rust
    // [TIP;NR] declarare
    let tablou: [i32;5] = [1, 2, 3, 4, 5];

    for i in 0..tablou.len() {
        print!("{:?} ", tablou[i]);
    }
    // print all
    println!("{:?}", tablou);
    println!("Element[0] = {}", tablou[0]);

```

### Functii
> O funcție **admite** zero sau mai mulți parametri.

> În acest exemplu, funcția `add` admite doi parametri de tip `i32` (număr întreg cu semn cu dimensiune de 32 de biți).

> La **returnarea unei expresii** se poate omite cuvântul cheie return și simbolul punct-virgulă de la final, așa cum s-a procedat în funcția subtract.

> **Numele funcțiilor** sunt mereu scrise în format `snake_case`.

```rust
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
}

```

### Returnarea mai multor valori
> Funcțiile pot returna mai multe valori prin r**eturnarea unui tuplu de valori**.

> Elementele unui tuplu pot fi accesate folosind indexul acestora. `(ex: my_tuple.0)`

```rust
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // returnează un tuplu de valori
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructurează tuplul în două variabile
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
```

### Return fara valoare

> Dacă pentru o funcție **NU se specifică ce tip returnează**, aceasta va **returna** un **tuplu gol**, cunoscut și sub `numele de unitate (unit)`.

> Un **tuplu gol** este reprezentat de `()`.

> Folosirea unui `()` nu este des întâlnită, dar va apărea de suficiente ori, deci este bine să știți ce se întâmplă.

```rust
fn make_nothing() -> () {
    return ();
}

// tipul pe care îl returnează este în mod implicit ()
fn make_nothing2() {
    // această funcție va returna (), dacă nu este specificat altceva pentru returnare
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // Afișarea unui text de depanare pentru a și b
    // Pentru că e greu să printăm nimicul
    println!("Valoarea lui a: {:?}", a);
    println!("Valoarea lui b: {:?}", b);
}

```

---

## Capitol 2 - Control Flow
### if / else if / else 
> Condițiile nu au nevoie de paranteze! 

> Toți operatorii relaționali și logici funcționează la fel: `==`, `!=`, `<`, `>`, `<=`, `>=`, `!`, `||`, `&&`.

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("mai puțin de 42");
    } else if x == 42 {
        println!("egal cu 42");
    } else {
        println!("mare mare de 42");
    }
}
```

### Bucle infinite
> Rust face asta într-un mod foarte simplu.

> `break` vă va arunca în afara buclei când sunteți pregătit.

```rust
fn main(){
    let mut x = 0;
    loop {
        x+=1;
        if x % 2 == 0 {
            println!("x : {} e par.",x)
        }
        if x == 42 {
            break;
        }
    }
}
```

### while loop
> `while` vă lasă să adăugați o condiție logică unei bucle.

> Dacă condiția impusă buclei devine **falsă**, bucla se va **termina**.

```rust
fn main(){
    let mut x = 0;
    while x !=42{
        x+=3;
        println!("x in while e : {}", x);
    }
}
```

### for loop

> Bucla `for` din Rust e o îmbunătățire importantă. Ea **iterează** peste valorile oricărei expresii care poate fi transformată într-un **iterator**. Vă întrebați ce este un **iterator**? Un **iterator** este un obiect pe care îl puteți întreba "Care este următorul element pe care îl ai?" până când acesta nu mai are elemente.

> **Rust** poate crea ușor `iteratori` care generează o **secvență de numere întregi**.

> Operatorul `..` creează un **iterator** care generează numere **de la un număr până la alt număr**, din unu în unu, fără să îl includă pe cel din urmă.

> Operatorul `..=` creează un **iterator** care generează numere **de la un număr până la alt număr**, din unu în unu, **inclusiv** cel din urmă.

```rust
// Bucla FOR
fn main(){
    let mut x = 0;
    // de la 0 la 4 | 0 < 5
    for x in 0..5 {
        println!("x in 0..5 : {}", x);
    }

    // de la 0 la 5 | 0 <= 5
    for x in 0..=5 {
        println!("x in 0..=5 : {}", x);
    }
}
```

### match (fostul switch)
> `match` este **exhaustiv**, deci toate cazurile trebuie să fie abordate și implementate.

> **Matching**, combinat cu destructurarea datelor, este de departe unul din cele mai întâlnite șabloane de programare pe care le veți vedea în Rust.

```rust
fn main() {
    let x = 42;

    match x {
        0 => {
            println!("am găsit zero");
        }
        // putem face un caz pentru mai multe valori
        1 | 2 => {
            println!("am găsit 1 sau 2!");
        }
        // putem face un caz pentru o mulțime
        3..=9 => {
            println!("am găsit un număr între 3 și 9 inclusiv");
        }
        // putem pune numărul care respectă cazul într-o variabilă
        matched_num @ 10..=100 => {
            println!("am găsit numărul {} între 10 și 100!", matched_num);
        }
        // acesta este cazul implicit care trebuie să existe dacă
        // nu sunt abordate toate cazurile posibile
        _ => {
            println!("am găsit alt număr!");
        }
    }

```

### Returnarea unor valori dintr-o buclă

> `loop` poate fi oprit pentru a returna o valoare.

```rust
fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "am găsit 13";
        }
    };
    println!("v={}", v);
}

```

### Returnarea unor valori din expresii block

> `if-urile`, `match-urile`, funcțiile și domeniile bloc au un mod unic de a returna valori în Rust.

> **Dacă ultima instrucțiune** din interiorul unui `if`, `match`, `funcții` sau `domeniu bloc` este o expresie fără `;`, Rust o va returna sub forma unei valori din acel bloc. 

> Acesta este un mod foarte bun de a crea o logică concisă care returnează o valoare care poate fi pusă într-o variabilă nouă.

> Observăm cum acest lucru permite unui `if` să funcționeze ca o **expresie ternară concisă**.
```rust
fn example() -> i32 {
    let x = 42;
    // expresia ternară concisă
    let v = if x < 42 { -1 } else { 1 };
    println!("din if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "este un hotdog",
        // observați că acoladele sunt opționale când există
        // o expresie simplă de returnare
        _ => "nu este un hotdog",
    };
    println!("tipul de mâncare: {}", result);

    let v = {
        // Acest domeniu de vizibilitate ne permite să nu poluăm spațiul funcțiilor
        let a = 1;
        let b = 2;
        a + b
    };
    println!("din bloc: {}", v);

    // Modul idiomatic de a returna o valoare în Rust la sfâșitul unei funcții
    v + 4
}

fn main() {
    println!("din funcție: {}", example());
}

```

---

## Capitol 3 - Structuri de date

### Struct
> Un `struct` este o **colecție de câmpuri** (field-uri).

> **Câmpurile** sunt pe scurt date asociate unei structuri. Valorile lor pot fi de tip **primar** sau o **structură de date**.

Definiția `structurii` este ca o **matriță** pentru compilator pentru a ști cum să **aranjeze câmpurile în memorie** într-un mod compact.

```rust
struct Creatura {
    // String este o structură
    tip_animal: String,
    nume: String,
    brate: i32,
    picioare: i32,
    arma_de_aparare: String,
}
```

### Apelarea metodelor

> Spre deosebire de funcții, metodele sunt funcții asociate unui tip specific de date.

> **metode statice** — metode care aparțin unui tip de date și sunt apelate folosind operatorul `::`

> **metode ale instanței** — metode care aparțin unei instanțe a unui tip de date și sunt apelate folosind operatorul `.`

```rust
fn main() {
    // Folosim o metodă statică ca să creem o instanță String
    let s = String::from("Metoda statica pentru o instanta String");
    println!("Static s = {}", s);
    println!("Static::{}, metoda.instanta={}",s,s.len());
}
```

### Memorie
Aplicațiile scrise în Rust au 3 zone de memorie unde este stocată informație:

> **memoria pentru date** - pentru date care sunt de dimensiune fixă și sunt **statice** (adică mereu disponibile pe toată durata rulării aplicației). Considerați textul din programul dumneavoastră (ex: `"Hello World!"`): memoria ocupată (bytes) de acest text este citită dintr-un singur loc în cod deci poate fi stocat în această zonă de memorie. 

>Compilatoarele fac foarte multe optimizări pentru acest tip de date și folosirea lor în general este considerată foarte rapidă, pentru că locația lor este cunoscută și fixă.

> **memoria pentru stivă (stack)** - pentru date declarate ca variabile în interiorul unei funcții (`variabile locale`). Locația în memorie a acestor date nu se schimbă pe durata apelului funcției; datorită acestui lucru compilatoarele pot optimiza codul astfel încât datele din stivă se accesează foarte rapid.

> **memoria pentru alocare dinamică (heap)** - pentru date care sunt `create în timpul rulării aplicației`. Datele în această zonă de memorie pot fi **adăugate, mutate, șterse, redimensionate, etc.** 

> Din cauza naturii sale dinamice, este în general considerată mai lentă, dar această zonă permite utilizări mult mai creative ale memoriei. Când adăugăm date în această zonă de memorie, numim această operație **alocare (de memorie)**. Când ștergem date din această zonă de memorie, numim această operație **dealocare (de memorie)**.

### Crearea datelor in memorie

> Când **instanțiem** o **structură** în codul nostru, aplicația creează câmpurile de date unul lângă altul în memorie.

> Instanțiem o structură specificând toate valorile câmpurilor în interiorul

`NumeleStructurii { ... }`

> Câmpurile unei structuri sunt accesate folosind operatorul `.`


* Textul din interiorul ghilimelelor este o dată care poate fi numai citită (ex: "Ferris"), deci acesta este pus în **zona memoriei pentru date**.

* Apelul funcției `String::from` creează o structură de tip `String` ale cărei câmpuri sunt depuse, unul lângă altul, lângă câmpurile structurii, pe stivă. Un `String` reprezintă text care poate fi modificat în următoarele moduri:

    * Se alocă memorie pe **heap** pentru text și acolo va putea fi modificat
    * Stochează o referință la acea locație de pe heap în structura `String` (Mai multe despre acest concept în următoarele lecții)
* Așadar, cei doi prieteni ai noștri, Ferris și Sarah, sunt structuri de date care vor avea mereu locații fixe în aplicația noastră, deci vor fi puși în stivă.

```rust
struct CreaturăMarină {
    tip_animal: String,
    nume: String,
    nr_mâini: i32,
    nr_picioare: i32,
    armă: String,
}

fn main() {
    // datele din CreaturăMarină sunt pe stivă
    let ferris = CreaturăMarină {
        // Struct-ul String este de asemenea pe stivă,
        // dar ține o referință a informației pe heap
        tip_animal: String::from("crab"),
        nume: String::from("Ferris"),
        nr_mâini: 2,
        nr_picioare: 4,
        armă: String::from("ghiară"),
    };

    let sarah = CreaturăMarină {
        tip_animal: String::from("caracatiță"),
        nume: String::from("Sarah"),
        nr_mâini: 8,
        nr_picioare: 0,
        armă: String::from("creier"),
    };
    
    println!(
        "{} este {}. Are {} mâini, {} picioare, și {} pe post de armă",
        ferris.nume, ferris.tip_animal, ferris.nr_mâini, ferris.nr_picioare, ferris.armă
    );
    println!(
        "{} este {}. Are {} mâini, {} picioare. Nu are nicio armă..",
        sarah.nume, sarah.tip_animal, sarah.nr_mâini, sarah.nr_picioare
    );
}

```

### Structuri de tip TUPLU

> Puteti crea structuri care sunt folosite ca un tuplu.

```rust
struct Location(i32, i32);

fn main() {
    // Acesta este tot o structură pe stivă
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}
```

### Grupare semnatica a Struct like TUPLU

```rust
struct TupleStruct(i32, i32);
struct NormalStruct {
    a: i32,
    b: i32,
}
```

```rust
let ts = TupleStruct(1, 2);
let ns = NormalStruct { a: 1, b: 2 };

// shortcut to initialize the fields of a struct using the values of the
// fields of another struct
let ns2 = NormalStruct { a: 5, ..ns };
let ts2 = TupleStruct { 0: 1, ..ts }; // for TupleStruct it needs curly brackets
                                      // and implicit field names

// Atribuire 
let TupleStruct(x, y) = ts;
println!("x: {}, y: {}", x, y);

let NormalStruct { a, b } = ns;
println!("a: {}, b: {}", a, b);

// Accesare
println!("Accessing ts by name - {}{}", ts.0, ts.1);
println!("Accessing ns by name - {}{}", ns.a, ns.b);
```

* Named structs provide clarity by explicitly naming each field, making it easier to understand the purpose of each component. Tuple structs are often shorter and more concise than named structs, making them suitable for simple wrapper types. For this purpose rust-rocket web framework package uses tuple structs

```rust
#[derive(rocket_db_pools::Database)]

#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);
```

* **Semantic Grouping** when we represent RGB color values
```rust
struct Rgb(u8, u8, u8);
```

* Type alias for a 2D point using a tuple struct
```rust
struct Point(f64, f64);

let origin = Point(0.0, 0.0);

// Access fields of the tuple struct
println!("x: {}, y: {}", origin.0, origin.1);

```









