#! /bin/bash

set -eu

if ! command -v diesel --help 2>/dev/null; then
    echo "non trovato"
    cargo install diesel_cli
fi

diesel migration run --migration-dir="$(dirname "${0}")/../database/migrations/"
