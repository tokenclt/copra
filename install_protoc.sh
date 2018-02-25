#!/bin/bash
# Used by Travis CI

set -ex

error() {
    echo "$@" >&2
    exit 1
}

test -n "$PROTOBUF_VERSION" || error "PROTOBUF_VERSION is not set"
test -n "$INSTALL_PATH" || error "INSTALL_PATH is not set"

if [ ! -d $INSTALL_PATH ] ; then 
    curl -sL https://github.com/google/protobuf/archive/v$PROTOBUF_VERSION.tar.gz | tar zx

    cd protobuf-$PROTOBUF_VERSION
    ./autogen.sh
    ./configure --prefix=$INSTALL_PATH
    make && sudo make install
fi
