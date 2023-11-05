user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`

default:
    @just --list

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} \
                 -t {{user}}/{{name}}:latest \
                 .

push:
    docker push {{user}}/{{name}} --all-tags

buildx:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    docker buildx build \
           --push \
           --platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
           --tag {{user}}/{{name}}:{{version}} \
           --tag {{user}}/{{name}}:latest \
           .

run:
    docker run --rm \
               --init \
               --name {{name}} \
               --init \
               --env-file .env \
               -v ${PWD}/crontab.txt:/app/crontab.txt \
               {{user}}/{{name}}:{{version}}

sh:
    docker run --rm \
               -it \
               --name {{name}} \
               --init \
               --env-file .env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{version}} \
               sh

