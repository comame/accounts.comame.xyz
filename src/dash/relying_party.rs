use crate::data::oidc_relying_party::RelyingParty;

pub fn list() -> Vec<RelyingParty> {
    RelyingParty::list_all().into_iter().map(|rp| RelyingParty {
        client_id: rp.client_id,
        redirect_uris: rp.redirect_uris,
        hashed_client_secret: "".to_string(),
    }).collect()
}

pub struct RelyingPartyRawSecret {
    pub rp: RelyingParty,
    pub raw_secret: String,
}
pub fn create(client_id: &str) -> Result<RelyingPartyRawSecret, ()> {
    let party = RelyingParty::register(client_id)?;
    Ok(RelyingPartyRawSecret {
        rp: RelyingParty {
            client_id: client_id.to_string(),
            redirect_uris: vec![],
            hashed_client_secret: "".to_string(),
        },
        raw_secret: party,
    })
}

pub fn delete(client_id: &str) {
    RelyingParty::delete(client_id);
}

pub fn add_redirect_uri(client_id: &str, redirect_uri: &str) -> Result<(), ()> {
    let rp = RelyingParty::find(client_id);
    if rp.is_none() {
        return Err(());
    }
    let rp = rp.unwrap();

    rp.add_redirect_uri(redirect_uri)
}

pub fn remove_redirect_uri(client_id: &str, redirect_uri: &str) {
    let rp = RelyingParty::find(client_id);
    if let Some(rp) = rp {
        rp.remove_redirect_uri(redirect_uri);
    }
}
