#! /bin/bash

go build -o out .

pushd front
    npm ci
    npm run build
popd

go build -o out .

pushd dashboard
    pushd web
        npm ci
        npm run build
    popd
    go build -o out
popd
