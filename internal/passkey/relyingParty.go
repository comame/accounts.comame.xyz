package passkey

// .publicKey.rp.id に入る値 (おおむねホスト名) を返す
func RelyingPartyID() string {
	return "localhost"
}
