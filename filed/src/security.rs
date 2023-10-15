use argon2::{Argon2, Params};

pub fn get_argon2() -> Argon2<'static> {
    argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, Params::new(65535, 4, 4, Some(64)).unwrap())
}