use serde::{Deserialize, Serialize};

use crate::db::user_binding::{
    exists_user_binding, insert_user_binding, list_user_binding, remove_user_binding,
};

#[derive(Serialize, Deserialize)]
pub struct UserBinding {
    pub relying_party_id: String,
    pub user_id: String,
}

impl UserBinding {
    pub fn create(relying_party_id: &str, user_id: &str) -> Result<Self, ()> {
        let obj = Self {
            relying_party_id: relying_party_id.to_string(),
            user_id: user_id.to_string(),
        };
        let _r = insert_user_binding(&obj);
        Ok(obj)
    }

    pub fn remove(relying_party_id: &str, user_id: &str) {
        remove_user_binding(&UserBinding {
            relying_party_id: relying_party_id.to_string(),
            user_id: user_id.to_string(),
        });
    }

    pub fn exists(relying_party_id: &str, user_id: &str) -> Result<(), ()> {
        let result = exists_user_binding(relying_party_id, user_id);
        if result {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn list(relying_party_id: &str) -> Vec<Self> {
        list_user_binding(relying_party_id)
    }
}
