use rand::Rng;

fn generate_random_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut key = String::with_capacity(length);

    for _ in 0..length {
        // Generate a random bit (0 or 1)
        let bit: u8 = rng.gen_range(0..=1);
        key.push_str(&bit.to_string());
    }

    key
}

// Encrypt the message using the one-time pad cipher
fn one_time_pad_encrypt(message: &str, key: &str) -> String {
    if message.len() != key.len() {
        panic!("Key must be the same length as the message");
    }

    let ciphertext: String = message
        .chars()
        .zip(key.chars()) // zip function is used to pair elements from two iterators.
        .map(|(m, k)| ((m.to_digit(2).unwrap() ^ k.to_digit(2).unwrap()) as u8).to_string()) // map is a method on iterators that applies a closure
        .collect();

    ciphertext
}

// Decrypt the ciphertext using the one-time pad cipher
fn one_time_pad_decrypt(ciphertext: &str, key: &str) -> String {
    if ciphertext.len() != key.len() {
        panic!("Key must be the same length as the ciphertext");
    }

    let message: String = ciphertext
        .chars()
        .zip(key.chars())
        .map(|(c, k)| ((c.to_digit(2).unwrap() ^ k.to_digit(2).unwrap()) as u8).to_string())
        .collect();

    message
}

fn main() {
    let message = "1010110"; // binary message
    let key = generate_random_key(message.len());

    println!("Message : {}", message);
    println!("Key: {}", key);

    let ciphertext = one_time_pad_encrypt(&message, &key);
    println!("Ciphertext: {}", ciphertext);

    let decrypted_message = one_time_pad_decrypt(&ciphertext, &key);
    println!("Decrypted: {}", decrypted_message);

    // Verify that the decryption is correct 
    assert_eq!(message, decrypted_message, "Decryption failed !");
}
