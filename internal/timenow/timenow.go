package timenow

import "time"

func Now() time.Time {
	if c == nil {
		return time.Now()
	}

	return time.Unix(c.Unix(), 0)
}

type clock interface {
	// 1970/01/01 00:00:00 UTC からの経過秒数
	Unix() int64
}

var c clock = nil

func SetClockForTest(clock clock) {
	c = clock
}
