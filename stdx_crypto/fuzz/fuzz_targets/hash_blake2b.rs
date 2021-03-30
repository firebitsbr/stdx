#![no_main]

use libfuzzer_sys::fuzz_target;
use stdx_crypto;

fuzz_target!(|data: &[u8]| {
    let _ = stdx_crypto::hash::blake2b::hash(data, None).expect("error hashing data");
});
