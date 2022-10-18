use openssl::rsa::Rsa;

use crate::{crypto::rand::random_str, db::rsa_keypair::get, enc::base64};

#[derive(Clone)]
pub struct RsaKeypair {
    pub public: String,
    pub private: String,
    pub kid: String,
}

impl RsaKeypair {
    pub fn new() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        let private = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        let public = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
        let kid = random_str(8);
        Self {
            private,
            public,
            kid,
        }
    }

    pub fn get() -> Self {
        get()
    }

    pub fn e(&self) -> String {
        let pubkey = Rsa::public_key_from_pem(self.public.as_bytes()).unwrap();
        base64::encode_base64_url(&pubkey.e().to_vec())
    }

    pub fn n(&self) -> String {
        let pubkey = Rsa::public_key_from_pem(self.public.as_bytes()).unwrap();
        base64::encode_base64_url(&pubkey.n().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{
        _test_init::init_mysql,
        rsa_keypair::{insert_force, insert_ignore},
    };

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
