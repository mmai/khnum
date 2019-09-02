#!/bin/sh

VERSION=0.1.0

mkdir $1 && cd $1
curl --location https://github.com/mmai/khnum/archive/v$VERSION.tar.gz | tar -xzv
mv khnum-$VERSION/* .
mv khnum-$VERSION/.* .
rmdir khnum-$VERSION 

echo "$1 project succesfully created"

