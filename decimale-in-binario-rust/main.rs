use std::io;

fn main() {
    println!("Inserisci il numero decimale da convertire in binario: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input");

    let numero_decimale: i32 = input.trim().parse().expect("Inserisci un numero intero valido");

    let mut numero_binario = String::new();
    let mut numero = numero_decimale;

    while numero > 0 {
        let resto = numero % 2;
        numero_binario.insert(0, (resto + 48) as char);
        numero /= 2;
    }

    println!("Il numero binario corrispondente è: {}", numero_binario);
}
