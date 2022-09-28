#! /bin/env node

const { readFileSync, readdirSync, writeFileSync, readFile } = require('fs')
const { execSync } = require('child_process')

const user = process.argv[2]
const password = process.argv[3]

if (!user || !password) throw './scripts/dev-sql.js MYSQL_USER MYSQL_PASSWORD'

const dotenvFile = readFileSync('./.env', { encoding: 'utf-8' })

const mysqlUser = /MYSQL_USER=(.+)/.exec(dotenvFile)?.[1]
const mysqlPassword = /MYSQL_PASSWORD=(.+)/.exec(dotenvFile)?.[1]
const mysqlDb = /MYSQL_DATABASE=(.+)/.exec(dotenvFile)?.[1]

if (!mysqlUser || !mysqlPassword || !mysqlDb) throw 'set .env'

const dir = readdirSync('./sql').sort()

for (const file of dir) {
    let replacedFile = readFileSync(`./sql/${file}`, { encoding: 'utf-8' })
    replacedFile = replacedFile.replace(/\$MYSQL_USER/g, mysqlUser)
    replacedFile = replacedFile.replace(/\$MYSQL_PASSWORD/g, mysqlPassword)
    replacedFile = replacedFile.replace(/\$MYSQL_DATABASE/g, mysqlDb)
    writeFileSync('/tmp/comame-dev-id-sql.sql', replacedFile)

    execSync(`mysql -hmysql.comame.dev -u${user} -p${password} < /tmp/comame-dev-id-sql.sql`)
}
