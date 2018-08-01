// ~/src/lib.rs

mod cchars;
use cchars::{encode_char, decode_cipher_char};

pub fn encode(plaintext: &str) -> String {
    let ciphertext: String = plaintext.chars().map(|c| encode_char(c).to_string() + " ").collect();
    ciphertext.trim_right().to_string()
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext.split_whitespace().map(|w| decode_cipher_char(w)).collect()
}

#[test]
fn encoding() {
    let plainttext = "Firefox";
    let expected = "··−· ·· ·−· · ··−· −−− −··−";
    let result = encode(plainttext);
    
    assert_eq!(expected, &result);
}

#[test]
fn decoding() {
    let ciphertext = "··−· ·· ·−· · ··−· −−− −··−";
    let expected = "FIREFOX";
    let result = decode(ciphertext);
    
    assert_eq!(expected, &result);
}
