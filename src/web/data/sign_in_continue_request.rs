use crate::data::authentication::AuthenticationMethod;
use crate::web::parse_form_urlencoded::parse;

#[derive(PartialEq, Eq, Debug)]
pub struct SignInContinueRequest {
    pub csrf_token: String,
    pub login_type: AuthenticationMethod,
    pub state_id: String,
    pub relying_party_id: String,
    pub user_agent_id: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SignInContinueNoSessionRequest {
    pub csrf_token: String,
    pub state_id: String,
}

impl SignInContinueRequest {
    pub fn parse_from(str: &str) -> Result<Self, ()> {
        let map = parse(str)?;

        let token = map.get("csrf_token");
        if token.is_none() {
            return Err(());
        }
        let token = token.unwrap().clone();

        let login_type = map.get("login_type");
        if login_type.is_none() {
            return Err(());
        }
        let login_type = AuthenticationMethod::parse(login_type.unwrap().as_str());
        if login_type.is_err() {
            return Err(());
        }

        let state_id = map.get("state_id");
        if state_id.is_none() {
            return Err(());
        }

        let relying_party_id = map.get("relying_party_id");
        if relying_party_id.is_none() {
            return Err(());
        }

        let user_agent_id = map.get("user_agent_id").cloned();
        if user_agent_id.is_none() {
            return Err(());
        }

        Ok(Self {
            csrf_token: token,
            login_type: login_type.unwrap(),
            state_id: state_id.unwrap().clone(),
            relying_party_id: relying_party_id.unwrap().clone(),
            user_agent_id: user_agent_id.unwrap(),
        })
    }
}

impl SignInContinueNoSessionRequest {
    pub fn parse_from(str: &str) -> Result<Self, ()> {
        let map = parse(str)?;

        let token = map.get("csrf_token");
        if token.is_none() {
            return Err(());
        }
        let token = token.unwrap().clone();

        let state_id = map.get("state_id");
        if state_id.is_none() {
            return Err(());
        }

        Ok(Self {
            csrf_token: token,
            state_id: state_id.unwrap().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SignInContinueRequest as Target;
    use crate::data::authentication::AuthenticationMethod;

    #[test]
    fn test() {
        assert_eq!(
            Target::parse_from(
                "csrf_token=abcde&login_type=password&state_id=xyz&relying_party_id=hoge&user_agent_id=ua"
            )
            .unwrap(),
            Target {
                csrf_token: "abcde".to_string(),
                login_type: AuthenticationMethod::Password,
                state_id: "xyz".to_string(),
                relying_party_id: "hoge".to_string(),
                user_agent_id: "ua".to_string(),
            }
        );
    }
}
