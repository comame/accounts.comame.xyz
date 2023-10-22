package db

import (
	"context"
	"database/sql"

	"github.com/comame/mysql-go"
)

func Conn() *sql.DB {
	return mysql.Conn()
}

func Begin(ctx context.Context) *sql.Tx {
	conn := mysql.Conn()
	tx, err := conn.BeginTx(ctx, nil)
	if err != nil {
		panic(err)
	}
	return tx
}

const DatetimeFormat = "2006-01-02 15:04:05"
