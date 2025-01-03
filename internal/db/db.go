package db

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/comame/readenv-go"

	_ "github.com/go-sql-driver/mysql"
)

var con *sql.DB

type envType struct {
	User     string `env:"MYSQL_USER"`
	Password string `env:"MYSQL_PASSWORD,optional"`
	Database string `env:"MYSQL_DATABASE"`
	Host     string `env:"MYSQL_HOST"`
	Port     string `env:"MYSQL_PORT,optional"`
}

func Initialize() {
	var env envType
	readenv.Read(&env)

	if env.Port == "" {
		env.Port = "3306"
	}

	dataSourceName := fmt.Sprintf(
		"%s:%s@(%s:%s)/%s",
		env.User,
		env.Password,
		env.Host,
		env.Port,
		env.Database,
	)
	if dataSourceName == "" {
		dataSourceName = fmt.Sprintf(
			"%s@(%s:%s)/%s",
			env.User,
			env.Host,
			env.Port,
			env.Database,
		)
	}

	conn, err := sql.Open("mysql", dataSourceName)
	if err != nil {
		panic(err)
	}

	conn.SetConnMaxLifetime(3 * time.Minute)
	conn.SetMaxOpenConns(10)
	conn.SetMaxIdleConns(10)

	con = conn
}

func Conn() *sql.DB {
	return con
}

func Begin(ctx context.Context) *sql.Tx {
	conn := Conn()
	tx, err := conn.BeginTx(ctx, nil)
	if err != nil {
		panic(err)
	}
	return tx
}

const DatetimeFormat = "2006-01-02 15:04:05"
