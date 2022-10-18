use crate::auth::password::calculate_password_hash;
use crate::data::oidc_relying_party::RelyingParty;

pub fn authenticate_client(client_id: &str, client_secret: &str) -> Result<(), ()> {
    let rp = RelyingParty::find(client_id);
    if rp.is_none() {
        return Err(());
    }
    let rp = rp.unwrap();

    if calculate_password_hash(&client_secret, client_id) != rp.hashed_client_secret {
        return Err(());
    }

    Ok(())
}
