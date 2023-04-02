pub mod password;

pub mod sign;

pub use password::{hash_password, verify_password};

pub fn new_rng() -> rand::rngs::StdRng {
    rand_core::SeedableRng::from_entropy()
}

pub fn encode_base64<T: AsRef<[u8]>>(data: T) -> String {
    use base64::{engine::general_purpose, Engine as _};

    general_purpose::STANDARD_NO_PAD.encode(data)
}

pub fn decode_base64<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{engine::general_purpose, Engine as _};

    general_purpose::STANDARD_NO_PAD.decode(data.as_ref())
}
