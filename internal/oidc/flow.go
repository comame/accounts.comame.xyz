package oidc

type Flow string

var (
	FlowUnused   Flow = ""
	FlowImplicit Flow = "implicit"
	FlowCode     Flow = "code"
)

func IdentFlowFromResponseType(responseType string) Flow {
	if responseType == "code" {
		return FlowCode
	}

	if responseType == "id_token" {
		return FlowImplicit
	}

	// 無意味な値を返しておく
	return FlowUnused
}
