package oidc

type Flow string

var (
	FlowUnused   Flow = ""
	FlowImplicit Flow = "implicit"
	FlowCode     Flow = "code"
)

func IdentFlowFromResponseType(responseType string) Flow {
	if responseType == "code" {
		return FlowImplicit
	}

	if responseType == "id_token" {
		return FlowCode
	}

	// 無意味な値を返しておく
	return FlowUnused
}
