package passkey

// navigator.credentials.create の引数を表す
type credentialCreateOptions struct {
	PublicKey publicKeyCredentialCreationOptions `json:"publicKey"`
}

type publicKeyCredentialCreationOptions struct {
	ChallengeBase64        string                                           `json:"challenge_base64"`
	AuthenticatorSelection publicKeyCredentialAuthenticatorSelectionOptions `json:"authenticatorSelection"`
	PubKeyCredParams       []publicKeyCredentialPubKeyCredParamsOptions     `json:"pubKeyCredParams"`
	RP                     publicKeyCredentialRPOptions                     `json:"rp"`
	User                   publicKeyCredentialUserOptions                   `json:"user"`
	Attestation            string                                           `json:"attestation"`
	ExcludeCredentials     []publicKeyCredentialExcludeCredentialsOptions   `json:"excludeCredentials,omitempty"`
}

type publicKeyCredentialAuthenticatorSelectionOptions struct {
	AuthenticatorAttachment string `json:"authenticatorAttachment"`
	RequireResidentKey      bool   `json:"requireResidentKey"`
	ResidentKey             string `json:"residentKey"`
	UserVerification        string `json:"userVerification"`
}

type publicKeyCredentialPubKeyCredParamsOptions struct {
	Type string    `json:"type"`
	Alg  algorithm `json:"alg"`
}

type publicKeyCredentialRPOptions struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

type publicKeyCredentialUserOptions struct {
	IDBase64    string `json:"id_base64"`
	Name        string `json:"name"`
	DisplayName string `json:"displayName"`
}

type publicKeyCredentialExcludeCredentialsOptions struct {
	Type     string `json:"type"`
	IDBase64 string `json:"id_base64"`
}

// navigator.credentials.create の返り値を表す
// TODO: 今後 Public にしておく必要はないと思うので、private に戻しておく
type publicKeyCredentialAttestation struct {
	Type     string                           `json:"type"`
	ID       string                           `json:"id"`
	RawID    string                           `json:"rawId"`
	Response authenticatorAttestationResponse `json:"response"`
}

type authenticatorAttestationResponse struct {
	ClientDataJSON string `json:"clientDataJSON"`
	// base64uri エンコードされた AuthenticatorAttestationResponse.getPublicKey()
	PublicKey string `json:"publicKey"`
	// AuthenticatorAttestationResponse.getPublicKeyAlgorithm()
	PublicKeyAlgorithm algorithm `json:"publicKeyAlgorithm"`
	// AuthenticatorAttestationResponse.getTransports()
	Transports []string `json:"transports"`

	clientData authenticatorResponseClientData
}

// navigator.credentials.get の返り値を表す
type publicKeyCredentialAssertion struct {
	Type     string                         `json:"type"`
	ID       string                         `json:"id"`
	RawID    string                         `json:"rawId"`
	Response authenticatorAssertionResponse `json:"response"`
}

type authenticatorAssertionResponse struct {
	ClientDataJSON    string `json:"clientDataJSON"`
	Signature         string `json:"signature"`
	UserHandle        string `json:"userHandle"`
	AuthenticatorData string `json:"authenticatorData"`

	clientData authenticatorResponseClientData
}

// navigator.credentials.create の引数を表す
type credentialGetOptions struct {
	PublicKey publicKeyCredentialRequestOptions `json:"publicKey"`
}

type publicKeyCredentialRequestOptions struct {
	ChallengeBase64  string `json:"challenge_base64"`
	RPID             string `json:"rpId"`
	UserVerification string `json:"userVerification"`
}

// AuthenticatorAttestationResponse.clientDataJSON
type authenticatorResponseClientData struct {
	Type        string `json:"type"`
	Challenge   string `json:"challenge"`
	Origin      string `json:"origin"`
	CrossOrigin bool   `json:"crossOrigin"`
}
