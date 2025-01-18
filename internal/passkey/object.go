package passkey

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
