#!/bin/env bash

JNI_DIR="./android/app/src/main/jniLibs" 

# Build using cargo-ndk
cargo ndk -t aarch64-linux-android --platform 21 \
 -t armv7-linux-androideabi --platform 21 \
 -t i686-linux-android --platform 21 \
 -t x86_64-linux-android --platform 21 \
 -o $JNI_DIR \
 build -p fhe_enc --release \

echo "Build completed and .so files copied successfully!"
