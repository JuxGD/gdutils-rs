use rand::random_range;
use xor_cryptor_keylen::XORCryptor;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rust_string_random::{random as random_string, Options, RandWay};

pub fn generate(player_id: f32, key: String) -> String {

    let random_number_0 = random_range(0..1000000) as f32;
    let random_number_1 = (random_number_0 + (player_id / 10000.0)) as usize;

    let random_number_string = random_number_1.to_string();

    let buffer = random_number_string.as_bytes().to_vec();

    let encrypted_buffer = match XORCryptor::encrypt_v2(key.as_bytes(), buffer) {
        Ok(enc) => enc,
        Err(e) => {
            println!("Error: {}", e);
            return "1".to_string();
        }
    };

    let encrypted_string = String::from_utf8_lossy(&encrypted_buffer);
    let base64_encrypted_string = URL_SAFE.encode(encrypted_string.to_string().as_bytes());

    let rand_str_options = Options {
        rand: RandWay::NORMAL,
        numbers: None,
        letters: None,
        specials: None,
    };

    let random_string = random_string(5, rand_str_options);

    let chk = format!("{:?}{}", random_string, base64_encrypted_string);

    return chk;
}