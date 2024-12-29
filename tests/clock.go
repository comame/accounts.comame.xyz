package tests

import (
	"time"

	"github.com/comame/accounts.comame.xyz/timenow"
)

type c struct {
	unix int64
}

func (c c) Unix() int64 {
	return c.unix
}

func setTimeFreeze(datetime string) {
	t, err := time.ParseInLocation(time.DateTime, datetime, time.FixedZone("Asia/Tokyo", 9*3600))
	if err != nil {
		panic(err)
	}

	timenow.SetClockForTest(c{
		unix: t.Unix(),
	})
}
