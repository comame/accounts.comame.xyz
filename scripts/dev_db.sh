# /bin/bash

MYSQL_PORT=33063

stop_mysql_redis() {
    if [ -e .pid ]; then
        cat .pid | awk '{ print $2 }' | xargs kill
        rm .pid
    fi
}

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

    echo "mysql $MYSQL_PID" >> .pid
}

start_local_redis() {
    redis-server --port 33064 &
    local REDIS_PID=$!

    echo "redis $REDIS_PID" >> .pid
}

function wait_until_listen() {
    local last_status=1
    while [ $last_status -ne 0 ]; do
        nc -z -v localhost $1 > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            local last_status=0
        else
            local last_status=1
        fi
    done
}

if [ "$1" = 'stop' ]; then
    stop_mysql_redis
    exit
fi

# 起動中なら止める
stop_mysql_redis

# MySQL の起動
start_local_mysql
wait_until_listen 33063

# redis の起動
start_local_redis
wait_until_listen 33064


# 環境変数に読み込み
set -a
. .env
set +a

# 開発用データの生成
mysql -uroot -hlocalhost -P33063 --protocol tcp -e"DROP DATABASE IF EXISTS id_dev"
mysql -uroot -hlocalhost -P33063 --protocol tcp -e"CREATE DATABASE id_dev"
mysql -uroot -hlocalhost -P33063 --protocol tcp -Did_dev < ./tables.sql
