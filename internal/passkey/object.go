package passkey

// navigator.credentials.create の引数を表す
type credentialCreationOptions struct {
	PublicKey credentialCreationPublicKeyOptions `json:"publicKey"`
}

type credentialCreationPublicKeyOptions struct {
	ChallengeBase64        string                                          `json:"challenge_base64"`
	AuthenticatorSelection credentialCreationAuthenticatorSelectionOptions `json:"authenticatorSelection"`
	PubKeyCredParams       []credentialCreationPubKeyCredParamsOptions     `json:"pubKeyCredParams"`
	RP                     credentialCreationRPOptions                     `json:"rp"`
	User                   credentialCreationUserOptions                   `json:"user"`
	ExcludeCredentials     []credentialCreationExcludeCredentialsOptions   `json:"excludeCredentials,omitempty"`
}

type credentialCreationAuthenticatorSelectionOptions struct {
	AuthenticatorAttachment string `json:"authenticatorAttachment"`
	RequireResidentKey      bool   `json:"requireResidentKey"`
	ResidentKey             string `json:"residentKey"`
	UserVerification        string `json:"userVerification"`
}

type credentialCreationPubKeyCredParamsOptions struct {
	Type string `json:"type"`
	Alg  int    `json:"alg"`

	algName string
}

type credentialCreationRPOptions struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

type credentialCreationUserOptions struct {
	IDBase64    string `json:"id_base64"`
	Name        string `json:"name"`
	DisplayName string `json:"displayName"`
}

type credentialCreationExcludeCredentialsOptions struct {
	Type     string `json:"type"`
	IDBase64 string `json:"id_base64"`
}

// navigator.credentials.create の返り値を表す
// TODO: 今後 Public にしておく必要はないと思うので、private に戻しておく
type PublicCredentialAttestation struct {
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
	PublicKeyAlgorithm int `json:"publicKeyAlgorithm"`
	// AuthenticatorAttestationResponse.getTransports()
	Transports []string `json:"transports"`

	clientData authenticatorAttestationResponseClientData
}

// AuthenticatorAttestationResponse.clientDataJSON
type authenticatorAttestationResponseClientData struct {
	Type        string `json:"type"`
	Challenge   string `json:"challenge"`
	Origin      string `json:"origin"`
	CrossOrigin bool   `json:"crossOrigin"`
}
