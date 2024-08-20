use rand::prelude::SliceRandom;
use rand::thread_rng;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn generate_random_key() -> String {
    let mut chars: Vec<char> = ALPHABET.chars().collect();

    let mut rng = thread_rng();
    chars.shuffle(&mut rng);

    let string: String = chars.into_iter().collect();
    string
}

fn encrypt(plaintext: &str, key: &str) -> String {
    let mut ciphertext = String::new();

    for ch in plaintext.chars() {
        if let Some(index) = ALPHABET.find(ch.to_ascii_uppercase()) {
            ciphertext.push(key.chars().nth(index).unwrap_or('\0'));
        } else {
            ciphertext.push(ch);
        }
    }

    ciphertext
}

fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();

    for ch in ciphertext.chars() {
        if let Some(index) = key.find(ch.to_ascii_uppercase()) {
            plaintext.push(ALPHABET.chars().nth(index).unwrap_or('\0'));
        } else {
            plaintext.push(ch);
        }
    }

    plaintext
}

fn main() {
    println!("hehe");

    let plaintext = "HELLO WORLD!!";
    let key = generate_random_key();

    println!("Original Alphabet: {}", ALPHABET);
    println!("Substitution key : {}", key.chars().collect::<String>());

    let ciphertext = encrypt(plaintext, &key);
    println!("Plaintext:  {}", plaintext);
    println!("Ciphertext: {}", ciphertext);

    let decrypted_text = decrypt(&ciphertext, &key);
    println!("Decrypted Text: {}", decrypted_text);

    assert_eq!(plaintext, decrypted_text, "Decryption failed!!");

}
