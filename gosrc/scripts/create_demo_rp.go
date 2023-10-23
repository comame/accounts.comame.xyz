package scripts

import (
	"github.com/comame/accounts.comame.xyz/auth"
	"github.com/comame/accounts.comame.xyz/db"
)

func init() {
	register("create_demo_rp", CreateDemoRP, "")
}

func CreateDemoRP(args ...string) error {
	cs := auth.CalculatePasswordHash("demo", "demo.accounts.comame.dev")
	if err := db.RelyingParty_insert("demo.accounts.comame.dev", cs); err != nil {
		return err
	}

	if err := db.RoleAccess_insert("admin", "demo.accounts.comame.dev"); err != nil {
		return err
	}

	if err := db.RelyingParty_newRedirectURI("demo.accounts.comame.dev", "http://localhost:8080/dev/callback.html"); err != nil {
		return err
	}
	if err := db.RelyingParty_newRedirectURI("demo.accounts.comame.dev", "http://accounts.comame.xyz/dev/callback.html"); err != nil {
		return err
	}

	return nil
}
