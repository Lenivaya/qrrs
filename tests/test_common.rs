pub use rand::{Rng, thread_rng};
pub use rand::distributions::Alphanumeric;

pub use qrrs::*;

pub fn random_text() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
