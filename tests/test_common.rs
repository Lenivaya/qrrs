pub use qrrs::*;

pub use rand::distributions::Alphanumeric;
pub use rand::{thread_rng, Rng};

pub fn random_text() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
