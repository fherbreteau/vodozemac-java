#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Felix Hilgers <contact@fhilgers.com>
#
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

PROFILE="release"
PROFILE_PATH="release"

rm -rf dist

cargo build --target aarch64-apple-darwin --profile=$PROFILE
cargo build --target x86_64-apple-darwin --profile=$PROFILE

mkdir -p dist/shared/darwin-x86-64
mkdir -p dist/shared/darwin-aarch64

cp target/x86_64-apple-darwin/$PROFILE_PATH/libvodozemac.dylib dist/shared/darwin-x86-64/
cp target/aarch64-apple-darwin/$PROFILE_PATH/libvodozemac.dylib dist/shared/darwin-aarch64/
