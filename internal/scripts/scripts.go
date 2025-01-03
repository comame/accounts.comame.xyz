package scripts

import (
	"fmt"
	"os"
)

type scriptFunc = func(args ...string) error

var mscripts = make(map[string]scriptFunc)
var mhelp = make(map[string]string)

func register(name string, f scriptFunc, help string) {
	mscripts[name] = f
	mhelp[name] = help
}

func Run(name string, args ...string) {
	f, ok := mscripts[name]
	if !ok {
		fmt.Printf("script %s is not registerd\n", name)
		os.Exit(1)
	}

	if err := f(args...); err != nil {
		panic(err)
	}
}
