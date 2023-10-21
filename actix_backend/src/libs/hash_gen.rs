use rand::{Rng, distributions::Alphanumeric};

// fairly unique case sensitive string generator for the length of your choice
pub fn alphanumeric(hash_len: usize) -> String {
    let string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(hash_len)
        .map(char::from)
        .collect();
    string
}
