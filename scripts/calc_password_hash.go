package scripts

import (
	"fmt"

	"github.com/comame/accounts.comame.xyz/auth"
)

func init() {
	register("calc_password_hash", CalcPasswordHash, "")
}

func CalcPasswordHash(args ...string) error {
	p := args[0]
	salt := args[1]

	h := auth.CalculatePasswordHash(p, salt)
	fmt.Println(h)

	return nil
}
