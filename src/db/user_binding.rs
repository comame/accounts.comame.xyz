use mysql::{params, prelude::*};

use crate::data::user_binding::UserBinding;

use super::mysql::get_conn;

pub fn insert_user_binding(user_binding: &UserBinding) -> Result<(), ()> {
    let result = get_conn().unwrap().exec_drop(
        "INSERT INTO user_bindings (relying_party_id, user_id) VALUES (:rp, :id)",
        params! {
            "rp" => user_binding.relying_party_id.clone(),
            "id" => user_binding.user_id.clone(),
        },
    );

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn remove_user_binding(user_binding: &UserBinding) {
    get_conn()
        .unwrap()
        .exec_drop(
            "DELETE FROM user_bindings WHERE relying_party_id=:rp AND user_id=:id",
            params! {
                "rp" => user_binding.relying_party_id.clone(),
                "id" => user_binding.user_id.clone(),
            },
        )
        .unwrap()
}

pub fn exists_user_binding(relying_party_id: &str, user_id: &str) -> bool {
    let result = get_conn()
        .unwrap()
        .exec_map(
            "SELECT COUNT(*) FROM user_bindings WHERE relying_party_id=:rp AND user_id=:user",
            params! {
                "rp" => relying_party_id,
                "user" => user_id
            },
            |(count,): (usize,)| count,
        )
        .unwrap();

    result.first().cloned().unwrap() != 0
}

pub fn list_user_binding(relying_party_id: &str) -> Vec<UserBinding> {
    get_conn()
        .unwrap()
        .exec_map(
            "SELECT relying_party_id, user_id FROM user_bindings WHERE relying_party_id=:rp",
            params! {
                "rp" => relying_party_id
            },
            |(rp, user)| UserBinding {
                relying_party_id: rp,
                user_id: user,
            },
        )
        .unwrap()
}
