package db

import (
	"database/sql"
	"fmt"
	"time"

	"github.com/comame/readenv-go"
	_ "github.com/go-sql-driver/mysql"
)

type env_t struct {
	MySQLUser     string `env:"MYSQL_USER"`
	MySQLPassword string `env:"MYSQL_PASSWORD"`
	MySQLDatabase string `env:"MYSQL_DATABASE"`
	MySQLHost     string `env:"MYSQL_HOST"`
}

var DB *sql.DB
var env env_t

func init() {
	readenv.Read(&env)

	dataSourceName := fmt.Sprintf(
		"%s:%s@(%s)/%s",
		env.MySQLUser,
		env.MySQLPassword,
		env.MySQLHost,
		env.MySQLDatabase,
	)

	dbLocal, err := sql.Open("mysql", dataSourceName)
	if err != nil {
		panic(err)
	}

	DB = dbLocal

	DB.SetConnMaxLifetime(3 * time.Minute)
	DB.SetMaxOpenConns(10)
	DB.SetMaxIdleConns(10)
}
