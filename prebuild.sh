#! /bin/bash

pushd front
    npm ci
    npm run build
popd

cargo build -r

pushd dashboard
    go build -o out
popd
