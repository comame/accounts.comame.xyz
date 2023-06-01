package main

import (
	"context"
	"database/sql"

	"github.com/comame/accounts.comame.xyz/dashboard/core"
	"github.com/comame/accounts.comame.xyz/dashboard/db"
)

func createUser(ctx context.Context, userId string) error {
	db := db.DB

	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if _, err := tx.ExecContext(ctx, `
		INSERT INTO users
			(id)
		VALUES
			(?)
	`, userId); err != nil {
		return err
	}

	if _, err := tx.ExecContext(ctx, `
		INSERT INTO user_role
			(user_id, role)
		VALUES
			(?, 'everyone')
	`, userId); err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	return nil
}

type userWithPasswordSetting struct {
	UserId      string `json:"user_id"`
	HasPassword bool   `json:"has_password"`
}

type listUserResponse struct {
	Values []userWithPasswordSetting `json:"values"`
}

func listUser(ctx context.Context) (*listUserResponse, error) {
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

	return &listUserResponse{Values: users}, nil
}

func deleteUser(ctx context.Context, userId string) error {
	db := db.DB

	if _, err := db.ExecContext(ctx, `
		DELETE FROM users WHERE id=?
	`, userId); err != nil {
		return err
	}

	return nil
}

func changeUserPassword(ctx context.Context, userId string, password string) error {
	db := db.DB

	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if err := revokeSessionInTransaction(ctx, tx, userId); err != nil {
		return err
	}

	hash := core.CalculatePasswordHash(password, userId)

	if _, err := tx.ExecContext(ctx, `
		INSERT INTO user_passwords
			(user_id, hashed_password)
		VALUES
			(?, ?)
		ON DUPLICATE KEY UPDATE
			hashed_password=?
	`, userId, hash, hash); err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	return nil
}

func deleteUserPassword(ctx context.Context, userId string) error {
	db := db.DB

	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if err := revokeSessionInTransaction(ctx, tx, userId); err != nil {
		return err
	}

	if _, err := tx.ExecContext(ctx, `
		DELETE FROM user_passwords WHERE user_id=?
	`, userId); err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	return nil
}

type authenticationLog struct {
	Sub        string `json:"sub"`
	Aud        string `json:"aud"`
	Iat        int64  `json:"iat"`
	RemoteAddr string `json:"remote_addr"`
}

type listUserAuthenticationResponse struct {
	Values []authenticationLog `json:"values"`
}

func listUserAuthentication(ctx context.Context, userId string) (*listUserAuthenticationResponse, error) {
	db := db.DB

	rows, err := db.QueryContext(ctx, `
		SELECT sub, aud, UNIX_TIMESTAMP(iat), remote_addr
		FROM idtoken_issues
		WHERE
			sub=?
		ORDER BY iat DESC
	`, userId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	s := make([]authenticationLog, 0)

	for rows.Next() {
		var sub string
		var aud string
		var iat int64
		var remoteAddr string
		if err := rows.Scan(&sub, &aud, &iat, &remoteAddr); err != nil {
			return nil, err
		}

		s = append(s, authenticationLog{
			Sub:        sub,
			Aud:        aud,
			Iat:        iat,
			RemoteAddr: remoteAddr,
		})
	}

	return &listUserAuthenticationResponse{Values: s}, nil
}

type listUserRoleResponse struct {
	Values []string `json:"roles"`
	UserId string   `json:"user_id"`
}

func listUserRole(ctx context.Context, userId string) (*listUserRoleResponse, error) {
	db := db.DB

	rows, err := db.QueryContext(ctx, `
		SELECT ROLE FROM user_role WHERE user_id=?
	`, userId)
	if err != nil {
		return nil, err
	}

	roles := make([]string, 0)

	for rows.Next() {
		var role string
		if err := rows.Scan(&role); err != nil {
			return nil, err
		}

		roles = append(roles, role)
	}

	return &listUserRoleResponse{
		Values: roles,
		UserId: userId,
	}, nil
}

func setUserRole(ctx context.Context, userId string, roles []string) error {
	db := db.DB

	tx, err := db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if _, err := db.ExecContext(ctx, `
		DELETE FROM user_role WHERE user_id=?
	`, userId); err != nil {
		return err
	}

	// 遅いかもしれないが、そこまで大量の行を追加するわけではないので OK とする
	for _, role := range roles {
		if _, err := db.ExecContext(ctx, `
			INSERT IGNORE INTO user_role (user_id, role) VALUES (?, ?)
		`, userId, role); err != nil {
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	return nil
}

type userinfoResponse struct {
	Value string `json:"value"`
}

func getUserInfo(ctx context.Context, userId string) (*userinfoResponse, error) {
	db := db.DB

	row := db.QueryRowContext(ctx, `
		SELECT value FROM userinfo WHERE sub=?
	`, userId)

	var value string

	if err := row.Scan(&value); err != nil {
		if err == sql.ErrNoRows {
			return &userinfoResponse{Value: "{}"}, nil
		}
		return nil, err
	}

	return &userinfoResponse{Value: value}, nil
}

func revokeSessionInTransaction(ctx context.Context, tx *sql.Tx, userId string) error {
	if _, err := tx.ExecContext(ctx, `
		DELETE FROM sessions WHERE user_id=?
	`, userId); err != nil {
		return err
	}
	return nil
}
