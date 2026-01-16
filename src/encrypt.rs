use simple_crypt;
use base64::{Engine as _, engine::general_purpose};


pub fn encrypt(text: String, password: &str) -> String {
    let content = simple_crypt::encrypt(text.as_bytes(), password.as_bytes()).expect("Failed to encrypt");
    general_purpose::STANDARD.encode(&content).clone()
}

pub fn decrypt(text_base64: String, password: &str) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(text_base64).expect("Wrong");
    String::from_utf8(simple_crypt::decrypt(&decoded_bytes, password.as_bytes()).expect("Failed to decrypt")).expect("Error")
}
