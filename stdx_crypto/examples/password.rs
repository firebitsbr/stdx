use stdx_crypto::{kdf::argon2id, Error};

fn main() -> Result<(), Error> {
    stdx_crypto::init()?;

    let password = "Hello world!";
    let hash_str = argon2id::hash_password(
        password.as_bytes(),
        argon2id::OPSLIMIT_INTERACTIVE,
        argon2id::MEMLIMIT_INTERACTIVE,
    )?
    .to_string();

    // simulate db storage
    let hashed = argon2id::HashedPassword::from(hash_str.as_str());

    assert!(argon2id::verify_password(&hashed, password.as_bytes()));

    println!("hash verified: {}", hash_str);

    return Ok(());
}
