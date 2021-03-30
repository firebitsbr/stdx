use stdx_crypto::{hash::blake2b, Error};

fn main() -> Result<(), Error> {
    stdx_crypto::init().expect("error initializing stdx_crypto");

    let data = "Hello world!";
    let mut hash_state = blake2b::State::new(blake2b::DIGEST_512, None)?;
    hash_state.update(data.as_bytes()).expect("error updating hash state");
    let digest = hash_state.finalize().expect("error getting hash digest");

    println!("{}", hex::encode(digest));

    return Ok(());
}
