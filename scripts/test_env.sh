# /bin/bash

MYSQL_PORT=33063

start_local_mysql() {
    DATADIR="$(pwd)/.testdb"

    if [ -e $DATADIR ]; then
        # すでに mysqld が起動しているとき、再起動する
        PID_FILE=$(ls $DATADIR | grep .pid)
        if [ -n "$PID_FILE" ]; then
            kill $(cat "$DATADIR/$PID_FILE")
            while [ -e "$DATADIR/$PID_FILE" ]; do
                sleep 1
            done
        fi
    fi

    if [ ! -e $DATADIR ]; then
        # https://dev.mysql.com/doc/refman/8.0/ja/postinstallation.html
        mysqld --datadir="$DATADIR" --log-error="$DATADIR/mysql.log" --initialize-insecure
    fi

    # MySQL の起動
    rm -f $DATADIR/undo_001 $DATADIR/undo_002 # undo_00{1,2} を消しておかないと起動に失敗する
    mysqld --datadir="$DATADIR" --log-error="$DATADIR/mysql.log" --socket="$DATADIR/mysql.sock" --port=$MYSQL_PORT &
    local MYSQL_PID=$!

    echo "$MYSQL_PID"
}

function wait_until_start() {
    local last_status=1
    while [ $last_status -ne 0 ]; do
        nc -z -v localhost 33063 > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            local last_status=0
        else
            local last_status=1
        fi
    done
}

# MySQL の起動
start_local_mysql
wait_until_start

# 環境変数に読み込み
set -a
. .env
set +a

# 開発用データの生成
mysql -uroot -hlocalhost -P33063 --protocol tcp -e"DROP DATABASE IF EXISTS id_dev"
mysql -uroot -hlocalhost -P33063 --protocol tcp -e"CREATE DATABASE id_dev"
mysql -uroot -hlocalhost -P33063 --protocol tcp -Did_dev < ./tables.sql

go run . script setup_default admin dashboard_client_secret
go run . script create_demo_rp
