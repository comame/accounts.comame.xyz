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
	AuthenticatedAt uint64 `json:"authenticated_at"`
	CreatedAt       uint64 `json:"created_at"`
	Audience        string `json:"audience"`
	Subject         string `json:"subject"`
	UserAgentId     string `json:"user_agent_id"`
	// method       omitted
}

// src/data/oidc_relying_party.rs
type RelyingParty struct {
	ClientId           string   `json:"client_id"`
	RedirectUris       []string `json:"redirect_uris"`
	HashedClientSecret string   `json:"hashed_client_secret"`
}

// src/data/op_user.rs
type OpUser struct {
	UserId   string         `json:"user_id"`
	OpUserId string         `json:"op_user_id"`
	Op       OpenidProvider `json:"op"`
}

// src/data/openid_provider.rs
type OpenidProvider int

// src/data/openid_provider.rs
const (
	Google OpenidProvider = iota
)

// src/data/role_access.rs
type RoleAccess struct {
	Role           string `json:"role"`
	RelyingPartyId string `json:"relying_party_id"`
}

// src/data/role.rs
type Role struct {
	Name string `json:"name"`
}

// src/data/user_password.rs
type UserPassword struct {
	UserId         string `json:"user_id"`
	HashedPassword string `json:"hashed_password"`
}

// src/data/user_role.rs
type UserRole struct {
	UserId string `json:"user_id"`
	Role   string `json:"role"`
}

// src/data/user.rs
type User struct {
	Id string `json:"id"`
}
