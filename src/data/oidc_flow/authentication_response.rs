#[derive(Debug)]
pub enum AuthenticationResponse {
    Implicit(ImplicitFlowAuthenticationResponse),
    Code(CodeFlowAuthenticationResponse),
}

#[derive(Debug)]
pub struct ImplicitFlowAuthenticationResponse {
    pub id_token: String,
    pub state: Option<String>,
}

#[derive(Debug)]
pub struct CodeFlowAuthenticationResponse {
    pub code: String,
    pub state: Option<String>,
}
