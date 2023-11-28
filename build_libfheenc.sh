#!/bin/env bash

JNI_DIR="./android/app/src/main/jniLibs" 

# Build using cargo-ndk
cargo ndk -t aarch64-linux-android \
 -t armv7-linux-androideabi \
 -t i686-linux-android \
 -t x86_64-linux-android \
 -o $JNI_DIR \
 build -p fhe_enc --release \

echo "Build completed and .so files copied successfully!"
