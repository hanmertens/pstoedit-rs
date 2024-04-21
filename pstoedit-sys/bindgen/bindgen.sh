#!/bin/bash

cd "$(dirname "$0")" || exit 1

bindgen_opts=(
    # Process header as C
    bindings.h
    # This function is present in header file but not in shared library
    --blocklist-function ignoreVersionCheck
    # These type definitions are not used in the C API
    --blocklist-type '.*_func(tion)?'
    # Output to source directory
    --output ../src/bindings.rs
)

bindgen "${bindgen_opts[@]}"
