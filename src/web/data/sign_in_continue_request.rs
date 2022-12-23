use serde::Deserialize;

use crate::data::authentication::AuthenticationMethod;

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct SignInContinueRequest {
    pub csrf_token: String,
    pub login_type: AuthenticationMethod,
    pub state_id: String,
    pub relying_party_id: String,
    pub user_agent_id: String,
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct SignInContinueNoSessionRequest {
    pub csrf_token: String,
    pub state_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use super::SignInContinueRequest as Target;
    use crate::data::authentication::AuthenticationMethod;

    #[test]
    fn test() {
        let json_body = r#"{
            "csrf_token": "abcde",
            "login_type": "password",
            "state_id": "xyz",
            "relying_party_id": "hoge",
            "user_agent_id": "ua"
        }"#;
        let result: Target = from_str(json_body).unwrap();
        let expected = Target {
            csrf_token: "abcde".to_string(),
            login_type: AuthenticationMethod::Password,
            state_id: "xyz".to_string(),
            relying_party_id: "hoge".to_string(),
            user_agent_id: "ua".to_string(),
        };
        assert_eq!(expected, result)
    }
}
