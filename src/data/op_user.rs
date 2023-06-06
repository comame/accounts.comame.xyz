use super::openid_provider::OpenIDProvider;
use crate::db::op_user::OpUserDb;

pub struct OpUser {
    pub user_id: String,
    pub op_user_id: String,
    pub op: OpenIDProvider,
}

impl OpUser {
    pub fn get(op_user_id: &str, op: OpenIDProvider) -> Option<Self> {
        OpUserDb::get(op_user_id, op)
    }
}
