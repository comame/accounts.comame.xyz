package core_test

import (
	"testing"

	"github.com/comame/accounts.comame.xyz/dashboard/core"
)

func TestCalculatePasswordHash(t *testing.T) {
	password := "WAwFWLGj8X74p1BMz2gtONinTunRkXI4"
	salt := "test"
	hash := "65573D7EC8865481BF67D7E09A389DA93D2000C40874C68313ECFD76101A35A0"

	got := core.CalculatePasswordHash(password, salt)

	if hash != got {
		t.Errorf("Want: %s, Got: %s", hash, got)
	}
}
