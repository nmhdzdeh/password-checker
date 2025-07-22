use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_password(length: usize) -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    password
}