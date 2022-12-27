extern crate reqwest;
extern crate flac;

use std::fs::File;
use std::io::BufWriter;

fn main() {
    // Crea un client HTTP
    let client = reqwest::Client::new();

    // Scarica il contenuto del video da YouTube
    let mut res = client.get("https://youtu.be/dQw4w9WgXcQ")
        .send().unwrap();

    // Apri un file in scrittura
    let mut file = BufWriter::new(File::create("audio.flac").unwrap());

    // Decodifica il file audio e salvalo nel file
    flac::Decoder::new(res).unwrap().decode_to_writer(&mut file).unwrap();
}
