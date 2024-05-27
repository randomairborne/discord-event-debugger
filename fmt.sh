#!/bin/bash
set -eu

FILES=./events/*

rm -rf events-fmt

mkdir events-fmt

for f in $FILES
do
    filename=$(basename $f)
    cat events/$filename | jq > events-fmt/$filename
done