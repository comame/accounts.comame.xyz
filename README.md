## Setup

1. Redis サーバを立てる
1. MySQL サーバを立てる
1. .env を参考に環境変数を設定する
1. ./scripts/dev-sql を実行して、MySQL を設定する

## Scripts

### dev-sql

開発環境のデータベースを設定する。

```
dev-sql MYSQL_USER MYSQL_PASSWORD UNTIL_CURRENT

UNTIL_CURRENT: 本番環境で動いているマイグレーションまで構築する
```

### migration

.env.prod が必要。本番環境のデータベースに反映させる。かならず `sql/migration` をコミットすること

```
migration MYSQL_USER MYSQL_PASSWORD
```
