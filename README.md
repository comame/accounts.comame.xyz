# 前提

- mysql-server
- redis-server

# 開発環境の起動

1. `scripts/dev_db.sh`
1. `./prebuild.sh`
1. `set -a; . .env; set +a`
1. `readenv .env`
   - `set -a; . .env; set +a`
1. `go run .`

# テスト

## キャッシュを消す

1. `go clean -testcache`

## E2Eテスト (自動)

1. `scripts/dev_db.sh`
1. `readenv .env`
1. `go test ./`

## E2Eテスト (インタラクティブなものも含む)

1. `INTERACTIVE=1 go test ./ -v`

## TODO

- ダッシュボードの起動
