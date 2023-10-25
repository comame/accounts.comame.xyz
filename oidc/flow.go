package oidc

type Flow int

const (
	FlowCode Flow = iota
	FlowImplicit
)
