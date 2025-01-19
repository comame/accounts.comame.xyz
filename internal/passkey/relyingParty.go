package passkey

import "net/url"

// .publicKey.rp.id に入る値 (おおむねホスト名) を返す
func RelyingPartyID(origin string) string {
	u, err := url.Parse(origin)
	if err != nil {
		return ""
	}
	return u.Hostname()
}
