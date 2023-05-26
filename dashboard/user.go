package main

import (
	"context"
	"database/sql"

	"github.com/comame/accounts.comame.xyz/dashboard/db"
)

type userWithPasswordSetting struct {
	UserId      string `json:"user_id"`
	HasPassword bool   `json:"has_password"`
}

func listUser(ctx context.Context) ([]userWithPasswordSetting, error) {
	db := db.DB

	rows, err := db.QueryContext(ctx, `
		SELECT users.id, user_passwords.hashed_password
		FROM users
		LEFT JOIN user_passwords on users.id = user_passwords.user_id
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []userWithPasswordSetting

	for rows.Next() {
		var id string
		var hash sql.NullString
		if err := rows.Scan(&id, &hash); err != nil {
			return nil, err
		}

		users = append(users, userWithPasswordSetting{
			UserId:      id,
			HasPassword: hash.Valid,
		})
	}

	return users, nil
}
