//Per utilizzare questa libreria, puoi importarla nel tuo codice e chiamare le sue funzioni come segue:





use math::{sqrt, pow, log, cube, circle_area};

fn main() {
    let x = 2.0;
    let y = 3.0;
    let r = 1.0;

    println!("Radice quadrata di {}: {}", x, sqrt(x));
    println!("Potenza di {} elevato a {}: {}", x, y, pow(x, y));
    println!("Logaritmo di {}: {}", x, log(x));
    println!("Cubo di {}: {}", x, cube(x));
    println!("Area di un cerchio di raggio {}: {}", r, circle_area(r));
}
