#! /bin/bash

npm run build -w front

cargo build -r

cd dashboard
go build -o out
