#!/bin/sh

mkdir $1 && cd $1
curl https://github.com/mmai/khnum/archive/v0.1.0.tar.gz | tar -xzv

