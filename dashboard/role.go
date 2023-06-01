package main

import (
	"context"

	"github.com/comame/accounts.comame.xyz/dashboard/db"
)

func createRole(ctx context.Context, roleName string) error {
	db := db.DB
	if _, err := db.ExecContext(ctx, `
		INSERT INTO role (name) VALUES (?)
	`, roleName); err != nil {
		return err
	}
	return nil
}

func deleteRole(ctx context.Context, roleName string) error {
	db := db.DB
	if _, err := db.ExecContext(ctx, `
		DELETE FROM role WHERE name=?
	`, roleName); err != nil {
		return err
	}
	return nil
}

type listRolesResponse struct {
	Values []string `json:"values"`
}

func listRole(ctx context.Context) (*listRolesResponse, error) {
	db := db.DB

	rows, err := db.QueryContext(ctx, `
		SELECT name FROM role
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	roles := make([]string, 0)

	for rows.Next() {
		var name string
		if err := rows.Scan(&name); err != nil {
			return nil, err
		}
		roles = append(roles, name)
	}

	return &listRolesResponse{Values: roles}, nil
}
