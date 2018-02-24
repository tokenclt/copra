#!/bin/bash
# Used by Travis CI

set -ex

error() {
    echo "$@" > &2
    exit 1
}

if [ -n "$PROTOBUF_VERSION"]; then
    error "PROTOBUF_VERSION is not set"
fi

curl -sL https://github.com/google/protobuf/archive/v$PROTOBUF_VERSION.tar.gz | tar zx

cd protobuf-$PROTOBUF_VERSION

./autogen.sh
./configure --prefix=/home/travis && make && sudo make install