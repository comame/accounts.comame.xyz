package core

import (
	"encoding/json"
	"fmt"
)

func (p OpenidProvider) MarshalJSON() ([]byte, error) {
	var s string

	switch p {
	case Google:
		s = "google"
	}

	return json.Marshal(s)
}

func (p *OpenidProvider) UnmarshalJSON(b []byte) error {
	var s string
	if err := json.Unmarshal(b, &s); err != nil {
		return err
	}

	switch s {
	case "google":
		*p = Google
	default:
		return fmt.Errorf("invalid openid_provider value")
	}

	return nil
}
