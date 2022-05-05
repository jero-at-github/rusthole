use rand::Rng;
use std::error::Error;
use std::fs::File as StdFile;
use std::io::BufReader as StdBufReader;

pub fn print_secret_phrase(secret_phrase: &str) {
    println!("Rusthole code is: {}", secret_phrase);
    println!("On the other computer, please run:");
    println!();
    println!("rusthole receive {}", secret_phrase);
}

pub fn get_phrase() -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let json_file = StdFile::open("./bip39.json")?;
    let json_reader = StdBufReader::new(json_file);
    let bip39_list: serde_json::Value =
        serde_json::from_reader(json_reader).expect("JSON was not well-formatted");

    let phrase = format!(
        "{}-{}-{}",
        rng.gen_range(0..=9u8),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap(),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap()
    );

    Ok(phrase)
}
