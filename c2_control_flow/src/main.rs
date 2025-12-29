fn main() {
    // if / else if / else
    let x = 42;
    if x < 42 {
        println!("x is less than 42");
    } else if x == 42 {
            println!("x is equal to 42");
        }
        else {
            println!("mai mare ca si 42");
        }

    // Bulca Infinita
    let mut x = 0;
    loop {
        x+=1;
        if x % 2 == 0 {
            println!("x : {} e par.",x)
        }
        if x == 42{
            break;
        }
    }

    // Bucla WHILE
    let mut x = 0;
    while x !=42{
        x+=3;
        println!("x in while e : {}", x);
    }

    // Bucla FOR
    x = 0;
    // de la 0 la 4 | 0 < 5
    for x in 0..5 {
        println!("x in 0..5 : {}", x);
    }

    // de la 0 la 5 | 0 <= 5
    for x in 0..=5 {
        println!("x in 0..=5 : {}", x);
    }

    // Match case
    let x = 42;
    match x{
        0 => {
            println!("x este 0");
        }
        // putem face un caz pentru mai multe valori
        1 | 2 => {
            println!("x este 1 sau 2");
        }
        // putem face un caz pentru o mulțime
        3..=9 => {
            println!("x este intre 3 si 9");
        }
        nr_gasit @ 10..=100 => {
            println!("Am gasit nr {} intre 10 si 100.", nr_gasit);
        }
        // default
        _ => {
            println!("Am gasit alt numar!");
        }
    }

    // Returnarea unor valori dintr-o bucla
    let mut x = 0;
    let v = loop {
        x+=1;
        if x == 13 {
            break "am gasit 13";
        }
    };
    println!("v = {}",v);

    // Returnarea unor valori din expresii bloc
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

    let outside = example();
    println!("Valoarea lui example() = {}", outside);


}

