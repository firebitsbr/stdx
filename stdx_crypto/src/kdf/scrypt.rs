// //! `crypto_pwhash_scryptsalsa208sha256`, a particular combination of Scrypt, Salsa20/8
// //! and SHA-256

// use crate::rand;
// use libc::c_ulonglong;

// /// Number of bytes in a `Salt`.
// pub const SALTBYTES: usize = libsodium_sys::crypto_pwhash_scryptsalsa208sha256_SALTBYTES as usize;

// /// Number of bytes in a `HashedPassword`.
// pub const HASHEDPASSWORDBYTES: usize =
//     libsodium_sys::crypto_pwhash_scryptsalsa208sha256_STRBYTES as usize;

// /// All `HashedPasswords` start with this string.
// pub const STRPREFIX: &[u8] = libsodium_sys::crypto_pwhash_scryptsalsa208sha256_STRPREFIX;

// /// Safe base line for `OpsLimit` for interactive password hashing.
// pub const OPSLIMIT_INTERACTIVE: OpsLimit =
//     OpsLimit(libsodium_sys::crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_INTERACTIVE as usize);

// /// Safe base line for `MemLimit` for interactive password hashing.
// pub const MEMLIMIT_INTERACTIVE: MemLimit =
//     MemLimit(libsodium_sys::crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_INTERACTIVE as usize);

// /// `OpsLimit` for highly sensitive data.
// pub const OPSLIMIT_SENSITIVE: OpsLimit =
//     OpsLimit(libsodium_sys::crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_SENSITIVE as usize);

// /// `MemLimit` for highly sensitive data.
// pub const MEMLIMIT_SENSITIVE: MemLimit =
//     MemLimit(libsodium_sys::crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_SENSITIVE as usize);

// /// `OpsLimit` represents the maximum number of computations to perform when
// /// using the functions in this module.
// ///
// /// A high `OpsLimit` will make the functions
// /// require more CPU cycles
// #[derive(Copy, Clone)]
// pub struct OpsLimit(pub usize);

// /// `MemLimit` represents the maximum amount of RAM that the functions in this
// /// module will use, in bytes.
// ///
// /// It is highly recommended to allow the functions to use
// /// at least 16 megabytes.
// #[derive(Copy, Clone)]
// pub struct MemLimit(pub usize);

// new_type! {
//     /// `Salt` used for password hashing
//     public Salt(SALTBYTES);
// }

// new_type! {
//     /// `HashedPassword`is a password verifier generated from a password
//     ///
//     /// A `HashedPassword` is zero-terminated, includes only ASCII characters and can
//     /// be conveniently stored into SQL databases and other data stores. No
//     /// additional information has to be stored in order to verify the password.
//     public HashedPassword(HASHEDPASSWORDBYTES);
// }

// /// `gen_salt()` randombly generates a new `Salt` for key derivation
// ///
// /// THREAD SAFETY: `gen_salt()` is thread-safe provided that you have called
// /// `stdx_crypto::init()` once before using any other function from stdx_crypto.
// pub fn gen_salt() -> Salt {
//     let mut salt = Salt([0; SALTBYTES]);
//     rand::bytes_into(&mut salt.0);
//     salt
// }

// /// The `derive_from_password()` function derives a key from a password and a `Salt`
// ///
// /// The computed key is stored into key.
// ///
// /// `opslimit` represents a maximum amount of computations to perform. Raising
// /// this number will make the function require more CPU cycles to compute a key.
// ///
// /// `memlimit` is the maximum amount of RAM that the function will use, in
// /// bytes. It is highly recommended to allow the function to use at least 16
// /// megabytes.
// ///
// /// For interactive, online operations, `OPSLIMIT_INTERACTIVE` and
// /// `MEMLIMIT_INTERACTIVE` provide a safe base line for these two
// /// parameters. However, using higher values may improve security.
// ///
// /// For highly sensitive data, `OPSLIMIT_SENSITIVE` and `MEMLIMIT_SENSITIVE` can
// /// be used as an alternative. But with these parameters, deriving a key takes
// /// more than 10 seconds on a 2.8 Ghz Core i7 CPU and requires up to 1 gigabyte
// /// of dedicated RAM.
// ///
// /// The salt should be unpredictable. `gen_salt()` is the easiest way to create a `Salt`.
// ///
// /// Keep in mind that in order to produce the same key from the same password,
// /// the same salt, and the same values for opslimit and memlimit have to be
// /// used.
// ///
// /// The function returns `Ok(key)` on success and `Err(())` if the computation didn't
// /// complete, usually because the operating system refused to allocate the
// /// amount of requested memory.
// pub fn derive_from_password(
//     keylen: usize,
//     passwd: &[u8],
//     salt: &Salt,
//     OpsLimit(opslimit): OpsLimit,
//     MemLimit(memlimit): MemLimit,
// ) -> Result<Vec<u8>, ()> {
//     let mut key = vec![0u8; keylen];
//     if unsafe {
//         libsodium_sys::crypto_pwhash_scryptsalsa208sha256(
//             key.as_mut_ptr(),
//             keylen as c_ulonglong,
//             passwd.as_ptr() as *const _,
//             passwd.len() as c_ulonglong,
//             salt.0.as_ptr(),
//             opslimit as c_ulonglong,
//             memlimit,
//         )
//     } == 0
//     {
//         Ok(key)
//     } else {
//         Err(())
//     }
// }

// /// `derive_from_password_interactive()` is a shortcut function for `derive_from_password()` with
// /// interactive limits (i.e. using `derive_from_password()` with `OPSLIMIT_INTERACTIVE`
// /// and `MEMLIMIT_INTERACTIVE`)
// pub fn derive_from_password_interactive(keylen: usize, passwd: &[u8], salt: &Salt) -> Result<Vec<u8>, ()> {
//     derive_from_password(
//         keylen,
//         passwd,
//         salt,
//         OPSLIMIT_INTERACTIVE,
//         MEMLIMIT_INTERACTIVE,
//     )
// }

// /// `derive_from_password_sensitive()` is a shortcut function for `derive_from_password()` with
// /// sensitive limits (i.e. using `derive_from_password()` with `OPSLIMIT_SENSITIVE`
// /// and `MEMLIMIT_SENSITIVE`)
// pub fn derive_from_password_sensitive(keylen: usize, passwd: &[u8], salt: &Salt) -> Result<Vec<u8>, ()> {
//     derive_from_password(keylen, passwd, salt, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE)
// }

// /// The `pwhash()` returns a `HashedPassword` which
// /// includes:
// ///
// /// - the result of a memory-hard, CPU-intensive hash function applied to the password
// ///   `passwd`
// /// - the automatically generated salt used for the
// ///   previous computation
// /// - the other parameters required to verify the password: opslimit and memlimit
// ///
// /// `OPSLIMIT_INTERACTIVE` and `MEMLIMIT_INTERACTIVE` are safe baseline
// /// values to use for `opslimit` and `memlimit`.
// ///
// /// The function returns `Ok(hashed_password)` on success and `Err(())` if it didn't complete
// /// successfully
// pub fn pwhash(
//     passwd: &[u8],
//     OpsLimit(opslimit): OpsLimit,
//     MemLimit(memlimit): MemLimit,
// ) -> Result<HashedPassword, ()> {
//     let mut hp = HashedPassword([0; HASHEDPASSWORDBYTES]);
//     if unsafe {
//         libsodium_sys::crypto_pwhash_scryptsalsa208sha256_str(
//             hp.0.as_mut_ptr() as *mut _,
//             passwd.as_ptr() as *const _,
//             passwd.len() as c_ulonglong,
//             opslimit as c_ulonglong,
//             memlimit,
//         )
//     } == 0
//     {
//         Ok(hp)
//     } else {
//         Err(())
//     }
// }

// /// `pwhash_interactive()` is a shortcut function for `pwhash()` with
// /// interactive limits (i.e. using `pwhash()` with `OPSLIMIT_INTERACTIVE`
// /// and `MEMLIMIT_INTERACTIVE`)
// pub fn pwhash_interactive(passwd: &[u8]) -> Result<HashedPassword, ()> {
//     pwhash(passwd, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE)
// }

// /// `pwhash_sensitive()` is a shortcut function for `pwhash()` with
// /// sensitive limits (i.e. using `pwhash()` with `OPSLIMIT_SENSITIVE`
// /// and `MEMLIMIT_SENSITIVE`)
// pub fn pwhash_sensitive(passwd: &[u8]) -> Result<HashedPassword, ()> {
//     pwhash(passwd, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE)
// }

// /// `pwhash_verify()` verifies that the password `str_` is a valid password
// /// verification string (as generated by `pwhash()`) for `passwd`
// ///
// /// It returns `true` if the verification succeeds, and `false` on error.
// pub fn pwhash_verify(hp: &HashedPassword, passwd: &[u8]) -> bool {
//     unsafe {
//         libsodium_sys::crypto_pwhash_scryptsalsa208sha256_str_verify(
//             hp.0.as_ptr() as *const _,
//             passwd.as_ptr() as *const _,
//             passwd.len() as c_ulonglong,
//         ) == 0
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_derive_from_password() {
//         let salt = Salt([
//             0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
//             24, 25, 26, 27, 28, 29, 30, 31,
//         ]);
//         let pw = b"Correct Horse Battery Staple";
//         // test vector generated by using libsodium
//         let key_expected = [
//             0xf1, 0xbb, 0xb8, 0x7c, 0x43, 0x36, 0x5b, 0x03, 0x3b, 0x9a, 0xe8, 0x3e, 0x05, 0xef,
//             0xad, 0x25, 0xdb, 0x8d, 0x83, 0xb8, 0x3d, 0xb1, 0xde, 0xe3, 0x6b, 0xdb, 0xf5, 0x4d,
//             0xcd, 0x3a, 0x1a, 0x11,
//         ];
//         let key = derive_from_password(32, pw, &salt, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
//         assert_eq!(key, key_expected);
//     }

//     #[test]
//     fn test_pwhash_verify() {
//         use crate::rand;

//         for i in 0..32usize {
//             let pw = rand::bytes(i);
//             let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
//             assert!(pwhash_verify(&pwh, &pw));
//         }
//     }

//     #[test]
//     fn test_pwhash_verify_tamper() {
//         use crate::rand;

//         for i in 0..16usize {
//             let mut pw = rand::bytes(i);
//             let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
//             for j in 0..pw.len() {
//                 pw[j] ^= 0x20;
//                 assert!(!pwhash_verify(&pwh, &pw));
//                 pw[j] ^= 0x20;
//             }
//         }
//     }

//     #[cfg(feature = "serde")]
//     #[test]
//     fn test_serialisation() {
//         use randombytes::randombytes;
//         use test_utils::round_trip;
//         for i in 0..32usize {
//             let pw = randombytes(i);
//             let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
//             let salt = gen_salt();
//             round_trip(pwh);
//             round_trip(salt);
//         }
//     }
// }
