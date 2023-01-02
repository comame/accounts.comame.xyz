use serde::{Deserialize, Serialize};

use crate::db::federated_user_binding::{
    delete_federate_user_binding, exists_federated_user_binding, insert_federated_user_binding,
    list_federated_user_binding,
};

use super::openid_provider::OpenIDProvider;

#[derive(Serialize, Deserialize)]
pub struct FederatedUserBinding {
    pub relying_party_id: String,
    pub issuer: OpenIDProvider,
}

impl FederatedUserBinding {
    pub fn exists(relying_party_id: &str, issuer: OpenIDProvider) -> Result<(), ()> {
        let result = exists_federated_user_binding(relying_party_id, &issuer.to_string());
        if result {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn delete(&self) {
        let _result =
            delete_federate_user_binding(&self.relying_party_id, &self.issuer.to_string());
    }

    pub fn new(relying_party_id: &str, issuer: OpenIDProvider) {
        let _result = insert_federated_user_binding(relying_party_id, &issuer.to_string());
    }

    pub fn list(relying_party_id: &str) -> Vec<Self> {
        let result = list_federated_user_binding(relying_party_id);
        result.unwrap()
    }
}
