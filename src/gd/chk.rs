use rand::random_range;
use xor_cryptor_keylen::XORCryptor;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rust_string_random::{random as random_string, Options, RandWay};
use eyre::Report;

pub fn generate_reward_chk(player_id: i32, key: &str) -> Result<String, Report> {

    let random_number_0 = f64::from(random_range(0..=1_000_000));
    let random_number_1 = (random_number_0 + (f64::from(player_id) / 10000.0)) as usize;

    let random_number_string = random_number_1.to_string();

    let buffer = random_number_string.as_bytes().to_vec();

    let encrypted_buffer = XORCryptor::encrypt_v2(key.as_bytes(), buffer)?;

    let encrypted_string = String::from_utf8_lossy(&encrypted_buffer);
    let base64_encrypted_string = URL_SAFE.encode(String::from(encrypted_string).as_bytes());

    let rand_str_options = Options {
        rand: RandWay::NORMAL,
        numbers: None,
        letters: None,
        specials: None,
    };

    let random_string = random_string(5, rand_str_options).unwrap_or_else(|_| String::from("aaaaa"));

    Ok(format!("{random_string}{base64_encrypted_string}"))
}