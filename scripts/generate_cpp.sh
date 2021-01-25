#!/usr/bin/env bash

SCRIPTS_DIR=$(dirname "$0")
source $SCRIPTS_DIR/common.sh

if [ -z "$PROTOC" ]; then
    echo "use system protoc."
    PROTOC="protoc"
fi

if [ -z "$GRPC_CPP_PLUGIN" ]; then
    echo "use system protoc."
    GRPC_CPP_PLUGIN=`which grpc_cpp_plugin`
fi

echo "generate cpp code..."

KVPROTO_ROOT="$SCRIPTS_DIR/.."
GRPC_INCLUDE=.:../include

cd $KVPROTO_ROOT
rm -rf proto-cpp && mkdir -p proto-cpp
rm -rf cpp/kvproto && mkdir cpp/kvproto

cp proto/* proto-cpp/

sed_inplace '/gogo.proto/d' proto-cpp/*
sed_inplace '/option\ *(gogoproto/d' proto-cpp/*
sed_inplace -e 's/\[.*gogoproto.*\]//g' proto-cpp/*

push proto-cpp
${PROTOC} -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto || exit $?
${PROTOC} -I${GRPC_INCLUDE} --grpc_out ../cpp/kvproto --plugin=protoc-gen-grpc="${GRPC_CPP_PLUGIN}" *.proto || exit $?
pop

push include
${PROTOC} -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto google/api/http.proto google/api/annotations.proto || exit $?
pop

rm -rf proto-cpp
