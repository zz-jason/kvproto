cmake_minimum_required (VERSION 2.8)

set (CMAKE_CXX_STANDARD 17)

set(PROTO_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../proto")
set(OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/kvproto")

file(GLOB PROTO_FILES "${PROTO_DIR}/*.proto")
foreach(PROTO_FILE ${PROTO_FILES})
    get_filename_component (FILENAME ${PROTO_FILE} NAME_WLE)
    set(PROTO_SRC "${OUTPUT_DIR}/${FILENAME}.pb.cc")
    set(PROTO_HDR "${OUTPUT_DIR}/${FILENAME}.pb.h")
    set(GRPC_SRC "${OUTPUT_DIR}/${FILENAME}.grpc.pb.cc")
    set(GRPC_HDR "${OUTPUT_DIR}/${FILENAME}.grpc.pb.h")

    list(APPEND OUTPUT_SOURCES ${PROTO_SRC} ${PROTO_HDR} ${GRPC_SRC} ${GRPC_HDR})
endforeach()

add_custom_command(
    OUTPUT ${OUTPUT_SOURCES}
    COMMAND PROTOC=${_PROTOBUF_PROTOC} ./generate_cpp.sh
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/../scripts
    DEPENDS ${PROTO_FILE}
)

add_library(kvproto ${OUTPUT_SOURCES})
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/kvproto)
target_include_directories(kvproto PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
