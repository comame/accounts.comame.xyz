use openssl::rsa::Rsa;

use crate::db::rsa_keypair::get;

#[derive(Clone)]
pub struct RsaKeypair {
    pub public: String,
    pub private: String,
}

impl RsaKeypair {
    pub fn new() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        let private = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        let public = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
        Self {
            private,
            public
        }
    }

    pub fn get() -> Self {
        get()
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{_test_init::init_mysql, rsa_keypair::{insert_ignore, insert_force}};

    use super::RsaKeypair;

    #[test]
    fn test() {
        init_mysql();
        let keypair = RsaKeypair::new();
        insert_force(&keypair);
        let db_keypair = RsaKeypair::get();
        assert_eq!(keypair.public, db_keypair.public);
        assert_eq!(keypair.private, db_keypair.private);
        insert_ignore(&RsaKeypair::get());
        let db_keypair = RsaKeypair::get();
        assert_eq!(keypair.public, db_keypair.public);
        assert_eq!(keypair.private, db_keypair.private);
    }
}
