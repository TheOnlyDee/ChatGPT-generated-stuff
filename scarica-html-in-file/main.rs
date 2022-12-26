use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Crea una nuova connessione TCP con example.com sulla porta 80
    let mut stream = TcpStream::connect("example.com:80")?;

    // Invia la richiesta HTTP GET per ottenere il contenuto HTML della homepage
    let request = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes())?;

    // Crea un nuovo file "sito.html" in scrittura
    let mut file = File::create("sito.html")?;
    let mut file_writer = BufWriter::new(&mut file);

    // Crea un buffer per leggere i dati ricevuti dalla connessione TCP
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();

    // Continua a leggere i dati dalla connessione finché non viene raggiunta la fine del flusso
    while reader.read_line(&mut buffer)? > 0 {
        file_writer.write_all(buffer.as_bytes())?;
        buffer.clear();
    }

    Ok(())
}
