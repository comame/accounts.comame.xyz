// テスト用の環境をセットアップする

package tests

import (
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/scripts"
)

func setup() error {
	if err := setupClearTable(); err != nil {
		return err
	}

	if err := scripts.SetupDefault("admin", "client_secret"); err != nil {
		return err
	}

	if err := scripts.CreateDemoRP(); err != nil {
		return err
	}

	if err := setupKeyPair(); err != nil {
		return err
	}

	return nil
}

func setupClearTable() error {
	rows, err := db.Conn().Query(`select table_name from information_schema.tables where table_schema = 'id_dev'`)
	if err != nil {
		return err
	}

	var tables []string

	for rows.Next() {
		var t string
		if err := rows.Scan(&t); err != nil {
			return err
		}
		tables = append(tables, t)
	}

	tx, err := db.Conn().Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback()

	for _, t := range tables {
		// 本来はクエリに直接文字列を埋め込むのは SQL Injection の危険性があるのでよくないが、以下の理由で許容する
		// - DBから取得したテーブル名をそのまま指定しているのでユーザー入力が紛れ込む余地がない
		// - 開発用データベースである
		// - テーブル名にはプレースホルダが利用できない
		if _, err := tx.Exec(`delete from ` + t); err != nil {
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	return nil
}

func setupKeyPair() error {
	if _, err := db.Conn().Exec(`
insert into rsa_keypair
(id, public, private, kid)
values
(
    1,
"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsicr/hIYRFhuKCQ+NBax
ZeNj3/yTcqu6PKVC7+9X6R74ITXTnweArSaoASKNyaWmanuEGvNmE4cZm3/4BMzJ
2YW6V5X6qj5xM7tolYAZ2VCCCcwPLbK54q3tgrzPZtPl/QpiJ4hCx17ajBvemOfS
OCXYoPy+IFL2ISiReqibiQJU7amZvVb+KmRnfCE2az+3Cj8D+fEd1SZQ+BWcDb36
L6Iw/1mvP5F58GrnkSwOQ2UvZESwR74DhlP2AoeTUw/lUbmgBLhz/P0Clk0RBuLE
HR6/VPLQVwa7PRSsawJKAAA1Ha9SCb9fWD6AJ6Q6pJ5taAqvwGe1w+wZOPMyTv0g
YwIDAQAB
-----END PUBLIC KEY-----",
"-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEAsicr/hIYRFhuKCQ+NBaxZeNj3/yTcqu6PKVC7+9X6R74ITXT
nweArSaoASKNyaWmanuEGvNmE4cZm3/4BMzJ2YW6V5X6qj5xM7tolYAZ2VCCCcwP
LbK54q3tgrzPZtPl/QpiJ4hCx17ajBvemOfSOCXYoPy+IFL2ISiReqibiQJU7amZ
vVb+KmRnfCE2az+3Cj8D+fEd1SZQ+BWcDb36L6Iw/1mvP5F58GrnkSwOQ2UvZESw
R74DhlP2AoeTUw/lUbmgBLhz/P0Clk0RBuLEHR6/VPLQVwa7PRSsawJKAAA1Ha9S
Cb9fWD6AJ6Q6pJ5taAqvwGe1w+wZOPMyTv0gYwIDAQABAoIBAG2KqoExzRwRJ8Kk
7l6G6ZNVqy6pllw2/W+Wyj7P80UTVszM1Q9+xH8zOrBf98DaiyYERqlvqf8t3fAA
UpdY+HA4yuhZ/uQ5Os/tVxQ9zScTWrH9eAPIVoXsHhN6VyjJ+CuL++iE31LJnyXx
aQCp4lfF5ZqvbZRgjpi64iEClYg7D+LzBuBNEo7RjUISEN0EbhOQ/rbpGUgw1zrI
M9o8Jk8IzkBKbJ7OkQXcYA9qQs+3q89tqBRejzKhNewfWc3nr7wuEMSRfDH7x41Y
ndAvmjpPjw/dgfkUZKT7pFYWY0IAtBqwCD21Dhn5L/2oltVRI8hR4gYxx+L9Wmjp
tdO0flkCgYEAwVyQcMSdikyMLh1BxanWrG1tYUmi1oOj8pC63D+wRa9eCWE9tJ65
1dr6o7tW8kz+yQANELrgI1VHQLR+c7Q0eIBOMXxiDiebeMpGtbx6IUYVpKsZqXg1
8FdmGaaV31B776QNucW6U2+0vywTGlJiDDPASuUC0oPIaA6Wk0NmwwcCgYEA691d
U7e6nBZIUB26lO2RJXJrbew1Mkamx8/x2odKEpQ1AF6E/4s91WrHqU90kfdp07+b
KzrSe0TzUZ01j+E41uEgB8WB3YfTaMsQgeXngX6ZYyPlLCO1r9Gsz8GURa8q4uoy
N6jtGpouMnOZlckhFlov9smRvPhH8fMvzB9PlMUCgYBuvDAMJM2EEmqFTkQIi0dh
4BkwChezehg+JhydXev5PIFCJepMskoC6zF26ybUBLw1KE5TMnKCSahQqg1w/da+
29vsAyu0p4ImHtF36sSWoahrcYF0yF87kRHrxrc1+MXBa9ZgeZhHiEWe5gLapCt6
iXiqa5S+MrJmxVP+ai9DqQKBgHPCqHxfLypOUV1oydswIc20M3+2r4EmZdKpf3UW
c0ddEApHWZUmHMny5110jqzZNkpjvt9ftlAjzhvfQZuFGWV1BkhqKku0zxCeoVJv
qMjIfrXGt0KLoC9ThDJPOttclnraIJ1qvjwRMd03GUkHdsLGrsW7tlh9rqnUBkBz
mZZVAoGAaqAMBgQpZ/ZGKwp45cNXd6TVCBfoaS5a+XB19Yjf/ELTrxP699zoPAwa
co3x+5Z7BeiG7vEg4D+HMxgaxQOQysklaF98knlLz2YGz3KWPqG4tPgiyPvakxlL
uIyrJoprYT/ZypCvUiEbbFDHOdeHEeFUHdOPltTEUZeQteq27IQ=
-----END RSA PRIVATE KEY-----",
"TNkhLzSn"
)
on duplicate key update
    public = values(public),
    private = values(private),
    kid = values(kid)
;
	`); err != nil {
		return err
	}

	return nil
}
