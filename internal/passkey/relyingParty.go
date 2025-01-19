package passkey

// .publicKey.rp.id に入る値 (おおむねホスト名) を返す
// TODO: env.HOST からうまいこと取れるようにする
func RelyingPartyID() string {
	return "localhost"
}
