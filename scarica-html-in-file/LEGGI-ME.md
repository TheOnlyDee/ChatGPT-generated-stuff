Il programma apre una nuova connessione TCP con "example.com" sulla porta 80, invia una richiesta HTTP GET per ottenere il contenuto HTML della homepage e salva i dati ricevuti in un file chiamato "sito.html".
Il programma va compilato usando un compiler, come cargo:

$ cargo build main.rs
$ ./main

oppure

$ cargo run main.rs
