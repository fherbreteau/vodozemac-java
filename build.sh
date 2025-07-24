#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Felix Hilgers <contact@fhilgers.com>
#
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

PROFILE="release"
PROFILE_PATH="release"

rm -rf dist

cargo build --target x86_64-unknown-linux-gnu --profile=$PROFILE
cargo build --target aarch64-unknown-linux-gnu --profile=$PROFILE
cargo build --target x86_64-pc-windows-gnu --profile=$PROFILE

mkdir -p dist/shared/linux-x86-64/
mkdir -p dist/shared/linux-aarch64/
mkdir -p dist/shared/win32-x86-64/

cp target/x86_64-unknown-linux-gnu/$PROFILE_PATH/libvodozemac.so dist/shared/linux-x86-64/
cp target/aarch64-unknown-linux-gnu/$PROFILE_PATH/libvodozemac.so dist/shared/linux-aarch64/
cp target/x86_64-pc-windows-gnu/$PROFILE_PATH/vodozemac.dll dist/shared/win32-x86-64/
