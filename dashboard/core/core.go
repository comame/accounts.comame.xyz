package core

import (
	"crypto/sha256"
	"encoding/hex"
)

// 認証基盤のコアになっているコード。ベースになっている型や重要なロジックをここに置く。
// Rust のコードに変更を加えたときには必ずここも変更すること。ただし、SQL で表現できるものはここに書かない。

// src/auth/password.rs
func CalculatePasswordHash(password string, salt string) string {
	withSalt := password + salt
	var bytes [32]byte

	for i := 0; i < 3; i += 1 {
		bytes = sha256.Sum256([]byte(withSalt))
	}

	return hex.EncodeToString(bytes[:])
}

// src/data/Authentication.rs
type Authentication struct {
	AuthenticatedAt uint64
	CreatedAt       uint64
	Audience        string
	Subject         string
	UserAgentId     string
	// method       omitted
}

// src/data/oidc_relying_party.rs
type RelyingParty struct {
	ClientId           string
	RedirectUris       []string
	HashedClientSecret string
}

// src/data/op_user.rs
type OpUser struct {
	UserId   string
	OpUserId string
	Op       OpenidProvider
}

// src/data/openid_provider.rs
type OpenidProvider int

// src/data/openid_provider.rs
const (
	Google OpenidProvider = iota
)

// src/data/role_access.rs
type RoleAccess struct {
	Role           string
	RelyingPartyId string
}

// src/data/role.rs
type Role struct {
	Name string
}

// src/data/user_password.rs
type UserPassword struct {
	UserId         string
	HashedPassword string
}

// src/data/user_role.rs
type UserRole struct {
	UserId string
	Role   string
}

// src/data/user.rs
type User struct {
	Id string
}
